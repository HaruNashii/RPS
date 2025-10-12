use sdl3:: 
{
    rect::Rect,
    sys::render::SDL_LOGICAL_PRESENTATION_STRETCH,
    ttf::Sdl3TtfContext,
    EventPump,
    render::{TextureCreator, Canvas},
    video::{WindowContext, Window},
};





pub const WINDOW_DEFAULT_SCALE: (u32, u32) = (1920, 1080);





pub fn create_window() -> (Canvas<Window>, EventPump, TextureCreator<WindowContext>, Sdl3TtfContext)
{
    let sdl_started = sdl3::init().unwrap();
    let video_system = sdl_started.video().unwrap();

    let mut window = video_system.window("Page System", WINDOW_DEFAULT_SCALE.0, WINDOW_DEFAULT_SCALE.1).resizable().position_centered().build().unwrap();
    window.set_minimum_size(1024, 576).unwrap();
    video_system.text_input().start(&window);

    let ttf_context = sdl3::ttf::init().unwrap();
    let event_pump = sdl_started.event_pump().unwrap();
    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    canvas.set_logical_size(WINDOW_DEFAULT_SCALE.0, WINDOW_DEFAULT_SCALE.1, SDL_LOGICAL_PRESENTATION_STRETCH).unwrap();
    canvas.set_viewport(Rect::new(0, 0, canvas.window().size().0, canvas.window().size().1));

    (canvas, event_pump, texture_creator, ttf_context)
}
