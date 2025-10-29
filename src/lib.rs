pub use crate::{
    misc::{center_elements::get_center, vec::GetOrCreate},
    system::{
        input_handler::{InputEvent, InputHandler},
        page_system::{Button, Page, PersistentElements},
        renderer::Renderer,
        state::AppState,
        window::{WINDOW_DEFAULT_SCALE, create_window, get_monitor_refresh_rate}
    }
};

pub mod misc;
pub mod sdl;
pub mod system;
