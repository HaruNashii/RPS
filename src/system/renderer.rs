use std::fmt::Debug;
use std::f64::consts::PI;
use crate::
{
    sdl::sdl3_generators::{GenImage, GenText},
    system::page_system::{Page, PageData, PersistentElements},
};
use sdl3::
{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::Sdl3TtfContext,
    video::{Window, WindowContext},
};



pub struct Renderer<'a, PageId, ButtonId> 
{ 
    _page_id: Option<PageId>,
    _button_id: Option<ButtonId>,
    pub canvas: &'a mut Canvas<Window>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub ttf_context: &'a Sdl3TtfContext,
}
impl<'a, PageId: Copy + Eq + Debug, ButtonId: Copy + Eq + Debug> Renderer<'a, PageId, ButtonId> 
{
    pub fn new(canvas: &'a mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl3TtfContext) -> Self { Self{ _page_id: None, _button_id: None, canvas, texture_creator, ttf_context} }

    /// Render All Pages
    pub fn render(&mut self, page_data: &PageData<PageId, ButtonId>, font_path: &String) 
    {
        if let Some(page) = &page_data.page_to_render
        {
            if page.has_persistent_elements.is_some() 
            { Renderer::render_page(self, page, Some(&page_data.persistent_elements_to_render), font_path);}
            else
            { Renderer::render_page(self, page, None, font_path); }
        }
    }
    
    pub fn render_page(&mut self, page: &Page<PageId, ButtonId>, persistent_elements: Option<&Vec<PersistentElements<PageId, ButtonId>>>, font_path: &String) 
    {
        match page.background_color 
        {
            Some(background_color) => 
            {
                self.canvas.set_draw_color(background_color);
                self.canvas.clear();
            }
    
            None => 
            {
                println!("Page, Without Background Color, Using Default Color: Black");
                self.canvas.set_draw_color(Color::BLACK);
                self.canvas.clear();
            }
        }
        Self::render_elements(self, Some(page), None, font_path);
        if let Some(new_persistent_elements) = persistent_elements {  for result in new_persistent_elements {Self::render_elements(self, None, Some(result), font_path);} }
        self.canvas.present();
    }

     pub fn draw_rounded_box(canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32, r: i32, color: Color,) 
     {
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(x + r, y, (w - 2 * r) as u32, h as u32)).unwrap();
        for &dx in &[0, w - r] {canvas.fill_rect(Rect::new(x + dx, y + r, r as u32, (h - 2 * r) as u32)).unwrap();}
        let steps = r * 4; 

        for i in 0..=steps 
        {
            let theta = (i as f64 / steps as f64) * (PI / 2.0);
            let offset_x = (r as f64 * theta.cos()).round() as i32;
            let offset_y = (r as f64 * theta.sin()).round() as i32;

            // Top-left
            canvas.draw_line((x + r - offset_x, y + r - offset_y),(x + r, y + r - offset_y),).unwrap();
            // Top-right
            canvas.draw_line((x + w - r - 1, y + r - offset_y),(x + w - r - 1 + offset_x, y + r - offset_y),).unwrap();
            // Bottom-left
            canvas.draw_line((x + r - offset_x, y + h - r - 1 + offset_y),(x + r, y + h - r - 1 + offset_y),).unwrap();
            // Bottom-right
            canvas.draw_line((x + w - r - 1, y + h - r - 1 + offset_y),(x + w - r - 1 + offset_x, y + h - r - 1 + offset_y),).unwrap();
        }
    }   

    fn render_elements(&mut self, page: Option<&Page<PageId, ButtonId>>, persistent_elements: Option<&PersistentElements<PageId, ButtonId>>, font_path: &String)
    {
        if let Some(page) = page
        { 
            if let Some(rects) = &page.rects { for (color, (rect, radius)) in rects { self.canvas.set_draw_color(*color); Self::draw_rounded_box(self.canvas, rect.x(), rect.y(), rect.width() as i32, rect.height() as i32, *radius, *color); } }
            if let Some(buttons) = &page.buttons { for tuple in buttons { if tuple.enabled { self.canvas.set_draw_color(tuple.color); Self::draw_rounded_box(self.canvas, tuple.rect.x(), tuple.rect.y(), tuple.rect.width() as i32, tuple.rect.height() as i32, tuple.radius, tuple.color); } } }
            if let Some(texts) = &page.texts 
            {
                let requisites = (texts, self.texture_creator, self.ttf_context);
                for tuple in requisites.generate_text(font_path)
                { 
                    self.canvas.copy(&tuple.0, None, tuple.1).unwrap_or_else(|err| {println!("text creator gives an error \nerror: {}\n", err);}); 
                }
            }
            if let Some(images) = &page.images 
            { 
                let requisites = (images, self.texture_creator);
                for tuple in requisites.generate_image()
                { 
                    self.canvas.copy(&tuple.0, None, tuple.1).unwrap_or_else(|err| { println!("image_creator creator gives an error \nerror: {}\n", err); }); 
                } 
            }
        }
        if let Some(persistent_elements) = persistent_elements
        {
            if let Some(rects) = &persistent_elements.rects { for (color, (rect, radius)) in rects { self.canvas.set_draw_color(*color); Self::draw_rounded_box(self.canvas, rect.x(), rect.y(), rect.width() as i32, rect.height() as i32, *radius, *color); } }
            if let Some(buttons) = &persistent_elements.buttons { for tuple in buttons { if tuple.enabled { self.canvas.set_draw_color(tuple.color); Self::draw_rounded_box(self.canvas, tuple.rect.x(), tuple.rect.y(), tuple.rect.width() as i32, tuple.rect.height() as i32, tuple.radius, tuple.color); } } }

            if let Some(texts) = &persistent_elements.texts 
            {
                let requisites = (texts, self.texture_creator, self.ttf_context);
                for tuple in requisites.generate_text(font_path)
                { 
                    self.canvas.copy(&tuple.0, None, tuple.1).unwrap_or_else(|err| {println!("text creator gives an error \nerror: {}\n", err);}); 
                }
            }
            if let Some(images) = &persistent_elements.images 
            { 
                let requisites = (images, self.texture_creator);
                for tuple in requisites.generate_image()
                { 
                    self.canvas.copy(&tuple.0, None, tuple.1).unwrap_or_else(|err| { println!("image_creator creator gives an error \nerror: {}\n", err); }); 
                } 
            }
        }
    }
}
