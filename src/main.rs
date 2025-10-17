use std::time::Duration;
use crate::
{
    actions::buttons_actions::button_action, 
    ui::pages::{ButtonId, PageId},
    system::
    {
        input_handler::{InputEvent, InputHandler},
        state::AppState,
        window::{create_window, get_monitor_refresh_rate, WINDOW_DEFAULT_SCALE},
    },
};





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
    let mut app_state = AppState { current_page: PageId::Page1 as usize, vec_user_input: Vec::new(), capturing_input: (false, None), window_size: WINDOW_DEFAULT_SCALE };

    //Populate Vec_Of_User_input With Page And Buttons That Receives User_Input
    app_state.push_vec_user_input(vec!
    [
        (PageId::Page1 as usize, ButtonId::ButtonPurpleInputStartPage1 as usize),
        (PageId::Page1 as usize, ButtonId::ButtonRedInputStartPage1 as usize),
        (PageId::Page2 as usize, ButtonId::ButtonPurpleInputStartPage2 as usize),
    ]);

    let refresh_rate = get_monitor_refresh_rate();
    'running: loop 
    {
        std::thread::sleep(Duration::from_millis(1000 / refresh_rate));
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
