use display_info::DisplayInfo;
use fontconfig::Fontconfig;
use std::fs;
use sdl3::
{
    rect::Rect, render::{Canvas, TextureCreator}, surface::Surface, sys::render::{SDL_RendererLogicalPresentation, SDL_LOGICAL_PRESENTATION_STRETCH}, ttf::Sdl3TtfContext, video::{Window, WindowContext}, EventPump
};





pub const WINDOW_DEFAULT_SCALE: (u32, u32) = (1920, 1080);





pub struct WindowConfig
{
    pub window_title: String,
    pub icon: (bool, Option<String>),
    pub start_window_size: (u32, u32),
    pub window_minimum_size: (u32, u32),
    pub resizable: bool,
    pub centered: bool,
    pub hint_sdl3_vsync: bool,
    pub different_sdl_presentation_mode: Option<SDL_RendererLogicalPresentation>,
    pub font: (String, Option<String>)
}

pub struct WindowModules
{
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
    pub ttf_context: Sdl3TtfContext,
    pub font_path: String, 
}


pub fn create_window(window_config: WindowConfig) -> WindowModules
{
    let sdl_started = sdl3::init().unwrap();
    let video_system = sdl_started.video().unwrap();
    let font_config = Fontconfig::new().expect("Failed To Start FontConfig");
    let font_info = font_config.find(&window_config.font.0, window_config.font.1.as_deref()).expect("Failed Find And Set Font With FontConfig");
    let font_path = font_info.path.display().to_string();
    let mut window_builder = video_system.window
    (
        &window_config.window_title,
        window_config.start_window_size.0,
        window_config.start_window_size.1,
    );

    if window_config.resizable { window_builder.resizable(); }
    if window_config.centered { window_builder.position_centered(); }
    if window_config.hint_sdl3_vsync { sdl3::hint::set(sdl3::hint::names::RENDER_VSYNC, "1"); };
    let mut window = window_builder.build().unwrap();

    if window_config.icon.0 
    {
        match window_config.icon.1
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
                            window.set_icon(&icon_surface);
                            drop(icon_surface); // Explicitly drop the surface
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

    window.set_minimum_size(window_config.window_minimum_size.0, window_config.window_minimum_size.1).unwrap();
    video_system.text_input().start(&window);

    let ttf_context = sdl3::ttf::init().unwrap();
    let event_pump = sdl_started.event_pump().unwrap();
    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    match window_config.different_sdl_presentation_mode
    {
        Some(sdl_presentation_mode) => canvas.set_logical_size(1920, 1080, sdl_presentation_mode).unwrap(),
        None => canvas.set_logical_size(1920, 1080, SDL_LOGICAL_PRESENTATION_STRETCH).unwrap()
    };
    canvas.set_viewport(Rect::new(0, 0, 1920, 1080));
    
    WindowModules{canvas, event_pump, texture_creator, ttf_context, font_path}
}
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

