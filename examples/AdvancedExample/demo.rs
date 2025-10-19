use std::time::Duration;
use rust_page_system::
{
    system::
    {
        input_handler::{InputEvent, InputHandler},
        state::AppState,
        window::{create_window, get_monitor_refresh_rate, WINDOW_DEFAULT_SCALE},
    },
};
use crate::
{   
    actions::buttons_actions::button_action, 
    ui::pages::{populate_or_update_app_state, PageId},
};





// To Be Ignored, Just An Setup To Configure Some Assets
use crate::build::setup_build;
mod build;





pub mod actions;
pub mod ui;




fn main() 
{
    // To Be Ignored, Just An Setup To Configure Some Assets
    setup_build();

    let (mut canvas, mut event_pump, texture_creator, ttf_context) = create_window(false);
    let input_handler = InputHandler;
    let mut app_state = AppState {current_page: (PageId::Page1 as usize, true), vec_user_input: Vec::new(), vec_user_input_string: Vec::new(), capturing_input: (false, None), window_size: WINDOW_DEFAULT_SCALE, persistent_elements: Vec::new(), all_pages: Vec::new() };
    populate_or_update_app_state(&mut app_state, false);

    let refresh_rate = get_monitor_refresh_rate();
    'running: loop 
    {
        std::thread::sleep(Duration::from_millis(1000 / refresh_rate));
        match input_handler.poll(&mut event_pump) 
        {
            InputEvent::Click(x, y)   => if let Some(button_id) = app_state.page_button_at(x, y) { button_action(&mut app_state, button_id); },
            InputEvent::Text(string)    => app_state.handle_text(string),
            InputEvent::Backspace               => app_state.handle_backspace(),
            InputEvent::Submit                  => app_state.submit_input(),
            InputEvent::Quit                    => break 'running,
            InputEvent::None                    => {}
        }
        populate_or_update_app_state(&mut app_state, true);
        app_state.render(&mut canvas, &texture_creator, &ttf_context);
    }
}
