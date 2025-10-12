use sdl3::
{
    rect::Rect,
    pixels::Color,
    ttf::Sdl3TtfContext,
    render::{TextureCreator, Canvas},
    video::{WindowContext, Window},
};
use crate::
{
    ui::pages::Page,
    sdl::sdl3_generators::{GenText, GenImage}
};





fn draw_filled_circle(canvas: &mut Canvas<Window>, cx: i32, cy: i32, r: i32, color: Color) 
{
    canvas.set_draw_color(color);
    for y in -r..=r 
    {
        for x in -r..=r 
        {
            if x * x + y * y <= r * r 
            {
                let _ = canvas.draw_point((cx + x, cy + y));
            }
        }
    }
}

fn draw_rounded_box(canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32, r: i32, color: Color)
{
    canvas.set_draw_color(color);
    canvas.fill_rect(Rect::new(x + r, y, (w - 2 * r) as u32, h as u32)).unwrap();

    for &dx in &[0, w - r] 
    {
        canvas.fill_rect(Rect::new(x + dx, y + r, r as u32, (h - 2 * r) as u32)).unwrap();
    }

    // Four corners (top-left, top-right, bottom-left, bottom-right)
    for &(ox, oy) in &[(r, r), (w - r - 1, r), (r, h - r - 1), (w - r - 1, h - r - 1)] 
    {
        draw_filled_circle(canvas, x + ox, y + oy, r, color);
    }
}

fn render_elements(canvas: &mut Canvas<Window>, page: &Page, texture_creator: &TextureCreator<WindowContext>, ttf_context: &Sdl3TtfContext) 
{
    if let Some(rects) = &page.rects      { for (color, (rect, radius)) in rects { canvas.set_draw_color(*color); draw_rounded_box(canvas, rect.x(), rect.y(), rect.width() as i32, rect.height() as i32, *radius, *color); } }
    if let Some(buttons) = &page.buttons  { for tuple in buttons { if tuple.enabled { canvas.set_draw_color(tuple.color); draw_rounded_box(canvas, tuple.rect.x(), tuple.rect.y(), tuple.rect.width() as i32, tuple.rect.height() as i32, tuple.radius, tuple.color); } } }
    if let Some(texts)   = &page.texts    { for tuple in (texts,  texture_creator, ttf_context).generate_text()  { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }
    if let Some(images)  = &page.images   { for tuple in (images, texture_creator).generate_image()              { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }
}

pub fn render_page(page: Page, persistent_page: Option<Page>, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, ttf_context: &Sdl3TtfContext)
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

    render_elements(canvas, &page, texture_creator, ttf_context);
    if let Some(ref new_persistent_page) = persistent_page { render_elements(canvas, new_persistent_page, texture_creator, ttf_context); }

    canvas.present();
}
