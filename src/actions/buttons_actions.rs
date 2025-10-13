use crate::{system::state::AppState, ui::pages::{ButtonId, ButtonId::*, PageId}};





pub fn button_action(app_state: &mut AppState, button_id: ButtonId)
{
        if !app_state.capturing_input
        {
            match button_id 
            {
                ButtonPage1 => app_state.current_page = PageId::Page1,
                ButtonPage2 => app_state.current_page = PageId::Page2,
                ButtonSubPage => app_state.current_page = PageId::Page2SubPage,
                ButtonBack => app_state.current_page = PageId::Page2,
                ButtonInputStart1 | ButtonInputStart2 => 
                {
                    app_state.capturing_input = true;
                }
            }
        }
}
