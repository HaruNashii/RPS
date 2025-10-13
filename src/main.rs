use std::time::Duration;
use crate::
{
    system::{input_handler::{InputEvent, InputHandler}, state::AppState, window::create_window},
};





pub mod sdl;
pub mod system;
pub mod actions;
pub mod ui;





fn main()
{
    let (mut canvas, mut event_pump, texture_creator, ttf_context) = create_window();
    let input_handler = InputHandler;
    let mut app_state = AppState::new();

    
    'running: loop 
    {
        std::thread::sleep(Duration::from_millis(32));
        app_state.window_size = (canvas.window().size().0, canvas.window().size().1);

        match input_handler.poll(&mut app_state, &mut event_pump) 
        {
            InputEvent::Click(button_id) => app_state.handle_action(button_id),
            InputEvent::Text(_) => {}, // already handled in state
            InputEvent::Submit => {},  // handled by state.submit_input()
            InputEvent::Quit => break 'running,
            InputEvent::None => {}
        }

        app_state.render(&mut canvas, &texture_creator, &ttf_context);
    }
}
