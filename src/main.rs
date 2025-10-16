use std::time::Duration;
use crate::
{
    actions::buttons_actions::button_action, 
    system::
    {
        input_handler::{InputEvent, InputHandler},
        state::AppState,
        window::create_window,
    },
};
/// Always Make "ButtonId" and "PageId" An Public Reimport In Main.rs For AppState Use
pub use ui::pages::{ButtonId, PageId};




pub mod actions;
pub mod sdl;
pub mod ui;
pub mod system;
pub mod tests;
pub mod misc;




fn main() 
{
    let (mut canvas, mut event_pump, texture_creator, ttf_context) = create_window(false);
    let input_handler = InputHandler;
    let mut app_state = AppState::new();

    //Populate Vec_Of_User_input With Page And Buttons That Receives User_Input
    app_state.push_vec_user_input(vec!
    [
        (PageId::Page1, ButtonId::ButtonPurpleInputStartPage1),
        (PageId::Page1, ButtonId::ButtonRedInputStartPage1),
        (PageId::Page2, ButtonId::ButtonPurpleInputStartPage2),
    ]);

    'running: loop 
    {
        std::thread::sleep(Duration::from_millis(32));
        match input_handler.poll(&mut event_pump) 
        {
            InputEvent::Click(x, y) => if let Some(button_id) = app_state.page_button_at(x, y) { button_action(&mut app_state, button_id); },
            InputEvent::Text(s) => app_state.handle_text(s),
            InputEvent::Backspace => app_state.handle_backspace(),
            InputEvent::Submit => app_state.submit_input(),
            InputEvent::Quit => break 'running,
            InputEvent::None => {}
        }
        app_state.render(&mut canvas, &texture_creator, &ttf_context);
    }
}
