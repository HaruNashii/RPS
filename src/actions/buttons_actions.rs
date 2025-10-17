use crate::
{
    ui::pages::{ButtonId::*, PageId},
    system::state::AppState
};




pub fn button_action(app_state: &mut AppState, button_id: usize) 
{
    if !app_state.capturing_input.0
    {
        if ButtonPage1 as usize   == button_id {app_state.current_page = PageId::Page1 as usize; return};
        if ButtonPage2 as usize   == button_id {app_state.current_page = PageId::Page2 as usize; return};
        if ButtonSubPage as usize == button_id {app_state.current_page = PageId::Page2SubPage as usize; return};
        if ButtonBack as usize    == button_id {app_state.current_page = PageId::Page2 as usize; return};
        // Non Handle Buttons Will Be Considered User Input Buttons
        app_state.capturing_input = (true, Some(button_id));
    }
}
