pub use crate::
{
    misc::
    {
        center_elements::get_center,
        vec::GetOrCreate
    },
    system::
    {
        renderer::Renderer,
        input_handler::{InputEvent, InputHandler},
        window::{create_window, get_monitor_refresh_rate, WINDOW_DEFAULT_SCALE},
        page_system::{Button, Page, PersistentElements},
        state::AppState,
    },
};



pub mod sdl;
pub mod system;
pub mod misc;
