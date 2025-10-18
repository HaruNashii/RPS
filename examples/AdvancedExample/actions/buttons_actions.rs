use rps::system::state::AppState;
use crate::ui::pages::{ButtonId::*, PageId};




pub fn button_action(app_state: &mut AppState, button_id: usize) 
{
    if !app_state.capturing_input.0
    {
        if ButtonPage1 as usize   == button_id {app_state.current_page.0 = PageId::Page1 as usize; app_state.current_page.1 = true; return};
        if ButtonPage2 as usize   == button_id {app_state.current_page.0 = PageId::Page2 as usize; app_state.current_page.1 = true; return};
        if ButtonSubPage as usize == button_id {app_state.current_page.0 = PageId::Page2SubPage as usize; return};
        if ButtonBack as usize    == button_id {app_state.current_page.0 = PageId::Page2 as usize; app_state.current_page.1 = true; return};
        // Non Handle Buttons Will Be Considered User Input Buttons
        app_state.capturing_input = (true, Some(button_id));
    }
}
