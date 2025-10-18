use rust_page_system::system::{state::AppState, window::WINDOW_DEFAULT_SCALE};
use crate::setup_tests::
{
    button_action,
    ButtonId,
    PageId,
};



#[cfg(test)]
mod setup_tests;




#[test]
fn app_state_initialization() 
{
    let state = AppState::new();
    assert_eq!(state.current_page, (1, false));
    assert_eq!(state.vec_user_input.len(), 0);
    assert_eq!(state.vec_user_input_string.len(), 0);
    assert_eq!(state.capturing_input, (false, None));
    assert_eq!(state.window_size, WINDOW_DEFAULT_SCALE);

}

#[test]
fn app_state_push_vec_user_input() 
{
    let mut state = AppState::new();
    state.push_vec_user_input(vec!
    [
        (PageId::Page1 as usize, ButtonId::ButtonPurpleInputStartPage1 as usize),
    ]);

    // Check The Len Of The Vec_User_Input
    assert_eq!(state.vec_user_input.len(), 1);
    // Check correct PageId and ButtonId
    assert_eq!(state.vec_user_input[0].1, PageId::Page1 as usize);
    assert_eq!(state.vec_user_input[0].2, ButtonId::ButtonPurpleInputStartPage1 as usize);
}

#[test]
fn app_state_handle_text_and_backspace() 
{
    let mut state = AppState::new();
    state.push_vec_user_input(vec!
    [
        (PageId::Page1 as usize, ButtonId::ButtonPurpleInputStartPage1 as usize),
    ]);

    state.current_page.0 = PageId::Page1 as usize;
    state.capturing_input = (true, Some(ButtonId::ButtonPurpleInputStartPage1 as usize));
    state.handle_text("hello".to_string());
    assert_eq!(state.vec_user_input[0].0, "hello");
    state.handle_backspace();
    assert_eq!(state.vec_user_input[0].0, "hell");
}

#[test]
fn app_state_submit_input() 
{
    let mut state = AppState::new();
    state.capturing_input = (true, Some(ButtonId::ButtonPurpleInputStartPage1 as usize));
    state.submit_input();
    assert!(!state.capturing_input.0);
    assert_eq!(None, state.capturing_input.1);
}

#[test]
fn app_state_handle_action_switch_page() 
{
    let mut state = AppState::new();
    // Test switching to Page1
    button_action(&mut state, ButtonId::ButtonPage1 as usize);
    assert_eq!(state.current_page.0, PageId::Page1 as usize);
    // Test switch to subpage
    button_action(&mut state, ButtonId::ButtonSubPage as usize);
    assert_eq!(state.current_page.0, PageId::Page1SubPage as usize);
    // Test Switch Back to Page1
    button_action(&mut state, ButtonId::ButtonBack as usize);
    assert_eq!(state.current_page.0, PageId::Page1 as usize);
}

#[test]
fn app_state_handle_action_starts_input_capture() 
{
    //Test Starting User Input
    let mut state = AppState::new();
    let input_button = ButtonId::ButtonPurpleInputStartPage1 as usize;
    button_action(&mut state, input_button);
    assert!(state.capturing_input.0);
    assert_eq!(state.capturing_input.1, Some(input_button));
}

#[test]
fn app_state_page_at_none_when_no_button() 
{
    let mut state = AppState::new();
    state.current_page.0 = PageId::Page1SubPage as usize;
    // These coordinates are likely not on any button
    let button_on_position = state.page_button_at(-10000.0, -10000.0);
    // See If Button Is Being Returned None When There Is No Button Selected
    assert!(button_on_position.is_none());
}
