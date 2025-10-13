use std::time::Duration;
use crate::system::
{
    input_handler::{InputEvent, InputHandler},
    state::AppState,
    window::create_window,
};





pub mod actions;
pub mod sdl;
pub mod system;
pub mod ui;






fn main() 
{
    let (mut canvas, mut event_pump, texture_creator, ttf_context) = create_window();
    let input_handler = InputHandler;
    let mut app_state = AppState::new();

    'running: loop 
    {
        std::thread::sleep(Duration::from_millis(32));
        match input_handler.poll(&mut event_pump) 
        {
            InputEvent::Click(x, y) => if let Some(button_id) = app_state.page_at(x, y) { app_state.handle_action(button_id); },
            InputEvent::Text(s) => app_state.handle_text(s),
            InputEvent::Backspace => app_state.handle_backspace(),
            InputEvent::Submit => app_state.submit_input(),
            InputEvent::Quit => break 'running,
            InputEvent::None => {}
        }
        app_state.render(&mut canvas, &texture_creator, &ttf_context);
    }
}
