use crate::ui::pages::{
    ButtonId::{self},
    PageId
};
use rust_page_system::system::{page_system::PageData, state::AppState};

pub fn button_action(app_state: &mut AppState<PageId, ButtonId>, button_id: &ButtonId, app_data: &mut PageData<PageId, ButtonId>)
{
    if !app_state.capturing_input.0
    {
        if &ButtonId::ButtonPage1 == button_id
        {
            app_state.change_current_page(app_data, PageId::Page1);
            return;
        };
        if &ButtonId::ButtonPage2 == button_id
        {
            app_state.change_current_page(app_data, PageId::Page2);
            return;
        };
        if &ButtonId::ButtonSubPage == button_id
        {
            app_state.change_current_page(app_data, PageId::Page2SubPage);
            return;
        };
        if &ButtonId::ButtonBack == button_id
        {
            app_state.change_current_page(app_data, PageId::Page2);
            return;
        };
        // Non Handle Buttons Will Be Considered User Input Buttons
        app_state.capturing_input = (true, Some(*button_id));
    }
}
