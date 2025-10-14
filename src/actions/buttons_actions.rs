use crate::
{
    system::state::AppState,
    system::page_system::{ButtonId, ButtonId::*, PageId},
};





pub fn button_action(app_state: &mut AppState, button_id: ButtonId) 
{
    if !app_state.capturing_input.0
    {
        match button_id 
        {
            ButtonPage1 =>   app_state.current_page = PageId::Page1,
            ButtonPage2 =>   app_state.current_page = PageId::Page2,
            ButtonSubPage => app_state.current_page = PageId::Page2SubPage,
            ButtonBack =>    app_state.current_page = PageId::Page2,
            // Non Handle Buttons Will Be Considered User Input Buttons
            _=> app_state.capturing_input = (true, Some(button_id)),
        }
    }
}
