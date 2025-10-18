use rust_page_system::system::state::AppState;



//==========================================================================================================================================================================
//===============================================================# can be a different file, like: buttons_actions.rs #======================================================
//==========================================================================================================================================================================
#[cfg(test)]
pub fn button_action(app_state: &mut AppState, button_id: usize) 
{
    if !app_state.capturing_input.0
    {
        if ButtonId::ButtonPage1 as usize   == button_id {app_state.current_page.0 = PageId::Page1 as usize; app_state.current_page.1 = true; return};
        if ButtonId::ButtonSubPage as usize == button_id {app_state.current_page.0 = PageId::Page1SubPage as usize; return};
        if ButtonId::ButtonBack as usize    == button_id {app_state.current_page.0 = PageId::Page1 as usize; app_state.current_page.1 = true; return};
        // Non Handle Buttons Will Be Considered User Input Buttons
        app_state.capturing_input = (true, Some(button_id));
    }
}



//==========================================================================================================================================================================
//===============================================================# can be a different file, like: pages.rs #================================================================
//==========================================================================================================================================================================
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Defines The ID for your Pages
pub enum PageId 
{
    Page1,
    Page1SubPage,
}
#[cfg(test)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(usize)]
/// Defines The ID for your Buttons
pub enum ButtonId 
{
    ButtonPage1,
    ButtonPurpleInputStartPage1,
    ButtonSubPage,
    ButtonBack,
}
