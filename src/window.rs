use sdl3::rect::Rect;
use sdl3::pixels::Color;
use sdl3::render::{TextureCreator, Canvas};
use sdl3::sys::render::SDL_LOGICAL_PRESENTATION_STRETCH;
use sdl3::video::{WindowContext, Window};
use sdl3::EventPump;
use crate::sdl3_generators::GenImage;
use crate::
{
    pages::Page,
    sdl3_generators::GenText
};





pub const WINDOW_DEFAULT_SCALE: (u32, u32) = (1920, 1080);





pub fn create_window() -> (Canvas<Window>, EventPump, TextureCreator<WindowContext>)
{
    let sdl_started = sdl3::init().unwrap();
    let video_system = sdl_started.video().unwrap();

    let mut window = video_system.window("Page System", WINDOW_DEFAULT_SCALE.0, WINDOW_DEFAULT_SCALE.1).resizable().position_centered().build().unwrap();
    window.set_minimum_size(1024, 576).unwrap();
    video_system.text_input().start(&window);

    let event_pump = sdl_started.event_pump().unwrap();

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    canvas.set_logical_size(WINDOW_DEFAULT_SCALE.0, WINDOW_DEFAULT_SCALE.1, SDL_LOGICAL_PRESENTATION_STRETCH).unwrap();
    canvas.set_viewport(Rect::new(0, 0, canvas.window().size().0, canvas.window().size().1));

    (canvas, event_pump, texture_creator)
}





fn draw_filled_circle(canvas: &mut Canvas<Window>, cx: i32, cy: i32, radius: i32, color: Color) 
{
    canvas.set_draw_color(color);
    for y in -radius..=radius 
    {
        for x in -radius..=radius 
        {
            if x * x + y * y <= radius * radius 
            {
                let _ = canvas.draw_point((cx + x, cy + y));
            }
        }
    }
}

fn draw_rounded_box(canvas: &mut Canvas<Window>, x: i32, y: i32, width: i32, height: i32, radius: i32, color: Color) 
{
    canvas.set_draw_color(color);

    canvas.fill_rect(Rect::new(x,                               y + radius,                              radius.try_into().unwrap(), (height - 2 * radius).try_into().unwrap())).unwrap(); // Left
    canvas.fill_rect(Rect::new(x + radius,                      y,                                       (width - 2 * radius).try_into().unwrap(), height.try_into().unwrap())).unwrap(); // Center
    canvas.fill_rect(Rect::new(x + width - radius, y + radius,  radius.try_into().unwrap(),              (height - 2 * radius).try_into().unwrap())).unwrap(); // Right

    draw_filled_circle(canvas,  x + radius,              y + radius,               radius,  color); // Top-left
    draw_filled_circle(canvas,  x + width - radius - 1,  y + radius,               radius,  color); // Top-right
    draw_filled_circle(canvas,  x + radius,              y + height - radius - 1,  radius,  color); // Bottom-left
    draw_filled_circle(canvas,  x + width - radius - 1,  y + height - radius - 1,  radius,  color); // Bottom-right
}






pub fn render_page(page: Page, persistent_page: Option<Page>, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>)
{
    canvas.set_draw_color(page.background_color.unwrap());
    canvas.clear();










    if let Some(rect_vector_of_tuple)    = &page.rects    { for tuple in rect_vector_of_tuple     { canvas.set_draw_color(tuple.0); draw_rounded_box(canvas, tuple.1.0.x(), tuple.1.0.y(), tuple.1.0.width() as i32, tuple.1.0.height() as i32, tuple.1.1, tuple.0); } }
    if let Some(buttons_vector_of_tuple) = &page.buttons  { for tuple in buttons_vector_of_tuple  { if tuple.0 { canvas.set_draw_color(tuple.1); draw_rounded_box(canvas, tuple.2.0.x(), tuple.2.0.y(), tuple.2.0.width() as i32, tuple.2.0.height() as i32, tuple.2.1, tuple.1); } } }
    if let Some(texts_vector_of_tuple)   = &page.texts    { for tuple in (texts_vector_of_tuple,  texture_creator).generate_text()  { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }
    if let Some(images_vector_of_tuple)  = &page.images   { for tuple in (images_vector_of_tuple, texture_creator).generate_image() { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }

    if let Some(persistent_elements) = persistent_page 
    {
        if let Some(rect_vector_of_tuple) =    &persistent_elements.rects   { for tuple in rect_vector_of_tuple    { canvas.set_draw_color(tuple.0); draw_rounded_box(canvas, tuple.1.0.x(), tuple.1.0.y(), tuple.1.0.width() as i32, tuple.1.0.height() as i32, tuple.1.1, tuple.0); } }
        if let Some(buttons_vector_of_tuple) = &persistent_elements.buttons { for tuple in buttons_vector_of_tuple { if tuple.0 { canvas.set_draw_color(tuple.1); draw_rounded_box(canvas, tuple.2.0.x(), tuple.2.0.y(), tuple.2.0.width() as i32, tuple.2.0.height() as i32, tuple.2.1, tuple.1); } } }
        if let Some(texts_vector_of_tuple) =   &persistent_elements.texts   { for tuple in (texts_vector_of_tuple,  texture_creator).generate_text()  { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }
        if let Some(images_vector_of_tuple) =  &persistent_elements.images  { for tuple in (images_vector_of_tuple, texture_creator).generate_image() { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }
    }

    canvas.present();
}
