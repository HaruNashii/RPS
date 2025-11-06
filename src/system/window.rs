use display_info::DisplayInfo;
use fontconfig::Fontconfig;
use sdl3::{
    EventPump, Sdl,
    clipboard::ClipboardUtil,
    iostream::IOStream,
    rect::Rect,
    render::{Canvas, TextureCreator},
    surface::Surface,
    sys::render::{SDL_LOGICAL_PRESENTATION_STRETCH, SDL_RendererLogicalPresentation},
    ttf::Sdl3TtfContext,
    video::{Window, WindowContext}
};
use std::fs;


pub const WINDOW_DEFAULT_SCALE: (u32, u32) = (1920, 1080);

pub struct WindowConfig<'a>
{
    pub window_title: String,
    pub icon: (Option<String>, Option<&'a include_dir::Dir<'a>>),
    pub start_window_size: (u32, u32),
    pub window_minimum_size: (u32, u32),
    pub resizable: bool,
    pub centered: bool,
    // pub hint_sdl3_vsync: bool,
    pub different_sdl_presentation_mode: Option<SDL_RendererLogicalPresentation>,
    pub font: (String, Option<String>)
}

pub struct WindowModules
{
    pub sdl_init: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
    pub ttf_context: Sdl3TtfContext,
    pub font_path: String,
    pub clipboard_system: ClipboardUtil,
    pub stretch_mode_status: bool
}

pub fn create_window(window_config: WindowConfig) -> WindowModules
{
    let sdl_init = sdl3::init().unwrap();
    let video_system = sdl_init.video().unwrap();
    let clipboard_system = video_system.clipboard();
    let font_config = Fontconfig::new().expect("Failed To Start FontConfig");
    let font_info = font_config.find(&window_config.font.0, window_config.font.1.as_deref()).expect("Failed Find And Set Font With FontConfig");
    let font_path = font_info.path.display().to_string();
    let mut window_builder = video_system.window(&window_config.window_title, window_config.start_window_size.0, window_config.start_window_size.1);

    let stretch_mode_status = if let Some(sdl_presentation_mode) = window_config.different_sdl_presentation_mode { sdl_presentation_mode == SDL_LOGICAL_PRESENTATION_STRETCH } else { true };

    if window_config.resizable
    {
        window_builder.resizable();
    }
    if window_config.centered
    {
        window_builder.position_centered();
    }
    // WARNING!!!: Current version of SDL3 is making the window have weird black bars when
    // resized in SDL_LOGICAL_PRESENTATION_STRETCH, please maintain it in false for now
    //if window_config.hint_sdl3_vsync { sdl3::hint::set(sdl3::hint::names::RENDER_VSYNC, "1"); };
    let mut window = window_builder.build().unwrap();

    // === Icon loading: Embedded first, fallback to local ===
    if let Some(icon_path) = window_config.icon.0
    {
        let lower = icon_path.to_lowercase();
        if lower.ends_with(".bmp")
        {
            // Try to find the icon in embedded assets first
            let normalized = {
                let path = std::path::Path::new(&icon_path);
                path.iter().filter_map(|c| c.to_str()).collect::<Vec<_>>().join("/")
            };
            if let Some(assets) = window_config.icon.1
                && let Some(file) = assets.get_file(&normalized)
            {
                let mut stream = IOStream::from_bytes(file.contents()).expect("Failed to create IOStream");
                let icon_surface = Surface::load_bmp_rw(&mut stream).expect("Failed to load embedded icon surface");
                window.set_icon(&icon_surface);
                println!("✅ Window icon loaded from embedded assets: {}", normalized);
                drop(icon_surface);
            }
            else if fs::exists(&icon_path).unwrap_or(false)
            {
                let icon_surface = Surface::load_bmp(icon_path.clone()).expect("Failed to load disk icon");
                window.set_icon(&icon_surface);
                println!("💾 Window icon loaded from disk: {}", icon_path);
                drop(icon_surface);
            }
            else
            {
                eprintln!("⚠️ Icon not found (not embedded, not on disk): {}", icon_path);
            }
        }
        else
        {
            eprintln!("⚠️ Icon must be a BMP file: {}", icon_path);
        }
    }

    window.set_minimum_size(window_config.window_minimum_size.0, window_config.window_minimum_size.1).unwrap();
    video_system.text_input().start(&window);

    let ttf_context = sdl3::ttf::init().unwrap();
    let event_pump = sdl_init.event_pump().unwrap();
    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();

    match window_config.different_sdl_presentation_mode
    {
        Some(sdl_presentation_mode) => canvas.set_logical_size(1920, 1080, sdl_presentation_mode).unwrap(),
        None => canvas.set_logical_size(1920, 1080, SDL_LOGICAL_PRESENTATION_STRETCH).unwrap()
    };
    canvas.set_viewport(Rect::new(0, 0, 1920, 1080));
    canvas.set_blend_mode(sdl3::render::BlendMode::Blend);
    WindowModules { sdl_init, canvas, event_pump, texture_creator, ttf_context, font_path, clipboard_system, stretch_mode_status }
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
