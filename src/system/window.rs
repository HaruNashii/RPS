use display_info::DisplayInfo;
use std::fs;
use sdl3::
{
    surface::Surface,
    rect::Rect,
    render::{Canvas, TextureCreator},
    sys::render::SDL_LOGICAL_PRESENTATION_STRETCH,
    ttf::Sdl3TtfContext,
    video::{Window, WindowContext},
    EventPump,
};





pub const WINDOW_DEFAULT_SCALE: (u32, u32) = (1920, 1080);



pub fn get_monitor_refresh_rate() -> u64
{
    let display_infos = DisplayInfo::all().unwrap();
    let mut all_monitors_refresh_rate = Vec::new();
    for display_info in display_infos 
    {
        all_monitors_refresh_rate.push(display_info.frequency as u64);
    }

    *all_monitors_refresh_rate.iter().max().unwrap()
}


pub fn create_window(window_is_hidden: bool, has_icon: (bool, Option<String>), hint_sdl3_vsync: bool) -> (Canvas<Window>, EventPump, TextureCreator<WindowContext>, Sdl3TtfContext)
{
    let sdl_started = sdl3::init().unwrap();
    let video_system = sdl_started.video().unwrap();

    if hint_sdl3_vsync 
    {
        // Enable vsync via hint before creating the canvas
        sdl3::hint::set(sdl3::hint::names::RENDER_VSYNC, "1");
    };

    //Flag Hidden Is Necessary For Some Unit Tests
    let mut window = if window_is_hidden
    {
        video_system.window("Page System", WINDOW_DEFAULT_SCALE.0, WINDOW_DEFAULT_SCALE.1).hidden().resizable().position_centered().build().unwrap()
    }
    else 
    {
        video_system.window("Page System", WINDOW_DEFAULT_SCALE.0, WINDOW_DEFAULT_SCALE.1).resizable().position_centered().build().unwrap()
    };

    if has_icon.0 
    {
        match has_icon.1
        {
            Some(icon_path) =>
            {
                if fs::exists(&icon_path).unwrap()
                {
                    if let Some((_, after)) = icon_path.rsplit_once('.') 
                    {
                        if after == "bpm"
                        {
                            let icon_surface = Surface::load_bmp(icon_path).unwrap();
                            window.set_icon(icon_surface);
                        }
                        else 
                        {
                            println!("WARNING!!!! Window is declared to have icon, but the provided icon path doesn't lead to an .bmp file");
                            println!("Icon Path Provided: {}", icon_path);
                        }
                    }
                    else 
                    {
                        println!("WARNING!!!! Window is declared to have icon, but the provided icon path doesn't lead to an .bmp file");
                        println!("Icon Path Provided: {}", icon_path);
                    }
                }
                else 
                {
                    println!("WARNING!!!! Window is declared to have icon, but icon path parsed doesn't exist");
                    println!("Icon Path Provided: {}", icon_path);
                }
            }
            None => { println!("WARNING!!!! Window is declared to have icon, but no icon path was parsed!!!!") }
        }
    }

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
