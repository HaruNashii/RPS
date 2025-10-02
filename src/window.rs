use sdl3::rect::Rect;
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





pub fn create_window() -> (Canvas<Window>, EventPump, TextureCreator<WindowContext>)
{
    let sdl_started = sdl3::init().unwrap();
    let video_system = sdl_started.video().unwrap();

    let mut window = video_system.window("Page System", 1920, 1080).resizable().position_centered().build().unwrap();
    window.set_minimum_size(1024, 576).unwrap();
    video_system.text_input().start(&window);

    let event_pump = sdl_started.event_pump().unwrap();

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    canvas.set_logical_size(1920, 1080, SDL_LOGICAL_PRESENTATION_STRETCH).unwrap();
    canvas.set_viewport(Rect::new(0, 0, canvas.window().size().0, canvas.window().size().1));

    (canvas, event_pump, texture_creator)
}





pub fn render_page(page: Page, persistent_page: Option<Page>, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>)
{
    canvas.set_draw_color(page.background_color.unwrap());
    canvas.clear();

    if let Some(rect_vector_of_tuple) =    &page.rects    { for tuple in rect_vector_of_tuple     { canvas.set_draw_color(tuple.0); canvas.fill_rect(tuple.1).unwrap(); } }
    if let Some(buttons_vector_of_tuple) = &page.buttons  { for tuple in buttons_vector_of_tuple  { if tuple.0 { canvas.set_draw_color(tuple.1); canvas.fill_rect(tuple.2).unwrap(); } } }
    if let Some(texts_vector_of_tuple) =   &page.texts    { for tuple in (texts_vector_of_tuple,  texture_creator).generate_text()  { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }
    if let Some(images_vector_of_tuple) =  &page.images   { for tuple in (images_vector_of_tuple, texture_creator).generate_image() { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }

    if let Some(persistent_elements) = persistent_page 
    {
        if let Some(rect_vector_of_tuple) =    &persistent_elements.rects   { for tuple in rect_vector_of_tuple    { canvas.set_draw_color(tuple.0); canvas.fill_rect(tuple.1).unwrap(); } }
        if let Some(buttons_vector_of_tuple) = &persistent_elements.buttons { for tuple in buttons_vector_of_tuple { if tuple.0 { canvas.set_draw_color(tuple.1); canvas.fill_rect(tuple.2).unwrap(); } } }
        if let Some(texts_vector_of_tuple) =   &persistent_elements.texts   { for tuple in (texts_vector_of_tuple,  texture_creator).generate_text()  { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }
        if let Some(images_vector_of_tuple) =  &persistent_elements.images  { for tuple in (images_vector_of_tuple, texture_creator).generate_image() { canvas.copy(&tuple.0, None, tuple.1).unwrap(); } }
    }

    canvas.present();
}
