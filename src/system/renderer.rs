use crate::
{
    sdl::sdl3_generators::{GenImage, GenText},
    system::page_system::{Page, PageData}, AppState,
};
use sdl3::
{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::Sdl3TtfContext,
    video::{Window, WindowContext},
};



pub struct Renderer<PageId, ButtonId> { _page_id: Option<PageId>, _button_id: Option<ButtonId>}
impl<PageId, ButtonId> Default for Renderer<PageId, ButtonId> where PageId: Copy + Eq, ButtonId: Copy + Eq, { fn default() -> Self { Self::new() } }
impl<PageId, ButtonId> Renderer<PageId, ButtonId> where PageId: Copy + Eq, ButtonId: Copy + Eq,
{
    pub fn new() -> Self { Self{ _page_id: None, _button_id: None } }

    /// Render All Pages
    pub fn render(&mut self, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, ttf_context: &Sdl3TtfContext, app_state: &AppState<PageId, ButtonId>,  page_data: &PageData<PageId, ButtonId>) 
    {
        let all_pages = &page_data.all_pages;
        let persistent_elements = &page_data.persistent_elements;
        let current_page = app_state.current_page;
        for page in all_pages
        {
            if current_page.0 == page.id && !page.has_persistent_elements.0 { Renderer::render_page(page, None, canvas, texture_creator, ttf_context); }
            if current_page.0 == page.id && page.has_persistent_elements.0 && let Some(vec_of_pageid) = &page.has_persistent_elements.1
            {
                let mut vec_persistent_elements = Vec::new();
                for (index, pageid) in vec_of_pageid.iter().enumerate()
                {
                    if *pageid == persistent_elements[index].id { vec_persistent_elements.push(&persistent_elements[index]); }
                }
                Renderer::render_page(page, Some(vec_persistent_elements), canvas, texture_creator, ttf_context);
            }
        }
    }

    fn draw_rounded_box(canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32, r: i32, color: Color) 
    {
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(x + r, y, (w - 2 * r) as u32, h as u32)).unwrap();
        for &dx in &[0, w - r] { canvas.fill_rect(Rect::new(x + dx, y + r, r as u32, (h - 2 * r) as u32)).unwrap(); }
        for &(ox, oy) in &[(r, r), (w - r - 1, r), (r, h - r - 1), (w - r - 1, h - r - 1)] { for cy in -r..=r { for cx in -r..=r { if cx * cx + cy * cy <= r * r { canvas.draw_point((x + ox + cx, y + oy + cy)).unwrap(); } } } }
    }
    
    fn render_elements(canvas: &mut Canvas<Window>, page: &Page<PageId, ButtonId>, texture_creator: &TextureCreator<WindowContext>, ttf_context: &Sdl3TtfContext)
    {
        if let Some(rects) = &page.rects 
        {
            for (color, (rect, radius)) in rects 
            {
                canvas.set_draw_color(*color);
                Self::draw_rounded_box(canvas, rect.x(), rect.y(), rect.width() as i32, rect.height() as i32, *radius, *color);
            }
        }
    
        if let Some(buttons) = &page.buttons 
        {
            for tuple in buttons 
            {
                if tuple.enabled 
                {
                    canvas.set_draw_color(tuple.color);
                    Self::draw_rounded_box(canvas, tuple.rect.x(), tuple.rect.y(), tuple.rect.width() as i32, tuple.rect.height() as i32, tuple.radius, tuple.color);
                }
            }
        }
    
        if let Some(texts) = &page.texts { for tuple in (texts, texture_creator, ttf_context).generate_text() { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }
        if let Some(images) = &page.images { for tuple in (images, texture_creator).generate_image() { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }
    }
    
    pub fn render_page(page: &Page<PageId, ButtonId>, persistent_page: Option<Vec<&Page<PageId, ButtonId>>>, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, ttf_context: &Sdl3TtfContext) 
    {
        match page.background_color 
        {
            Some(background_color) => 
            {
                canvas.set_draw_color(background_color);
                canvas.clear();
            }
    
            None => 
            {
                println!("Page, Without Background Color, Using Default Color: Black");
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();
            }
        }
        Self::render_elements(canvas, page, texture_creator, ttf_context);
        if let Some(new_persistent_page) = persistent_page { for result in new_persistent_page {Self::render_elements(canvas, result, texture_creator, ttf_context);} }
        canvas.present();
    }
}
