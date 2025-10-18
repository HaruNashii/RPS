pub use crate::
{
    misc::center_elements::get_center,
    system::
    {
        renderer::render_page,
        input_handler::{InputEvent, InputHandler},
        window::{create_window, get_monitor_refresh_rate, WINDOW_DEFAULT_SCALE},
        page_system::{Button, Page},
        state::AppState,
    },
};



pub mod sdl;
pub mod system;
pub mod misc;
