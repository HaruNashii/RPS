use rust_page_system::system::window::{WindowConfig, create_window};
use sdl3::sys::render::SDL_LOGICAL_PRESENTATION_STRETCH;

/// Minimal Wayland dock-mode demo.
/// Run with: cargo run --example DockExample --features wayland

fn main() {
    let window_config = WindowConfig {
        window_title: "RPS Dock Example".to_string(),
        icon: (None, None),
        start_window_size: (800, 48),
        window_minimum_size: (800, 48),
        resizable: false,
        centered: false,
        different_sdl_presentation_mode: Some(SDL_LOGICAL_PRESENTATION_STRETCH),
        font: ("JetBrainsMono".to_string(), None),
        wayland_dock_style: true,
    };
    let _window_modules = create_window(window_config);
    loop {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}