use rust_page_system::system::state::AppState;
use crate::ui::pages::{ButtonId::{self, *}, PageId};




pub fn button_action(app_state: &mut AppState<PageId, ButtonId>, button_id: &ButtonId) 
{
    if !app_state.capturing_input.0
    {
        if &ButtonPage1      == button_id {app_state.current_page = (PageId::Page1,        true);  return};
        if &ButtonPage2      == button_id {app_state.current_page = (PageId::Page2,        true);  return};
        if &ButtonSubPage    == button_id {app_state.current_page = (PageId::Page2SubPage, false); return};
        if &ButtonBack       == button_id {app_state.current_page = (PageId::Page2,        true);  return};
        // Non Handle Buttons Will Be Considered User Input Buttons
        app_state.capturing_input = (true, Some(*button_id));
    }
}
