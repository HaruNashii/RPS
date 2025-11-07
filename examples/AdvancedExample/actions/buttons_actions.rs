use crate::ui::pages::{
    ButtonId::{self},
    PageId, persistent_elements2
};
use rust_page_system::system::{page_system::PageData, state::AppState};

pub fn button_action(app_state: &mut AppState<PageId, ButtonId>, button_id: &ButtonId, app_data: &mut PageData<PageId, ButtonId>)
{
    if !app_state.capturing_input.0
    {
        if &ButtonId::ButtonPage1 == button_id
        {
            //this disable all eventpump events, it's just here for demonstration porpuse
            app_state.all_events_disable = true;
            app_data.forced_persistent_elements = Some(vec![persistent_elements2()]);
            app_state.change_current_page(app_data, PageId::Page1, button_id);
            app_state.all_events_disable = false;
            return;
        };
        if &ButtonId::ButtonPage2 == button_id
        {
            app_state.change_current_page(app_data, PageId::Page2, button_id);
            return;
        };
        if &ButtonId::ButtonSubPage == button_id
        {
            app_state.change_current_page(app_data, PageId::Page2SubPage, button_id);
            return;
        };
        if &ButtonId::ButtonBack == button_id
        {
            app_state.change_current_page(app_data, PageId::Page2, button_id);
            return;
        };
        // Non Handle Buttons Will Be Considered User Input Buttons
        app_state.capturing_input = (true, Some(*button_id));
    }
}
