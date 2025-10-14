use crate::
{
    actions::buttons_actions::button_action,
    system::state::AppState,
    system::page_system::{ButtonId, PageId}
};





#[test]
fn app_state_initialization() 
{
    let state = AppState::new();
    assert_eq!(state.current_page, PageId::Page1);
    assert_eq!(state.vec_user_input.len(), 0);
    assert_eq!(state.capturing_input, (false, None));
    assert_eq!(state.window_size, (1920, 1080));
}

#[test]
fn app_state_push_vec_user_input() 
{
    let mut state = AppState::new();
    state.push_vec_user_input(vec!
    [
        (PageId::Page1, ButtonId::ButtonPurpleInputStartPage1),
        (PageId::Page1, ButtonId::ButtonRedInputStartPage1),
        (PageId::Page2, ButtonId::ButtonPurpleInputStartPage2),
    ]);
    // Check The Len Of The Vec_User_Input
    assert_eq!(state.vec_user_input.len(), 3);
    // Check correct PageId and ButtonId
    assert_eq!(state.vec_user_input[0].1, PageId::Page1);
    assert_eq!(state.vec_user_input[0].2, ButtonId::ButtonPurpleInputStartPage1);

    assert_eq!(state.vec_user_input[1].1, PageId::Page1);
    assert_eq!(state.vec_user_input[1].2, ButtonId::ButtonRedInputStartPage1);

    assert_eq!(state.vec_user_input[2].1, PageId::Page2);
    assert_eq!(state.vec_user_input[2].2, ButtonId::ButtonPurpleInputStartPage2);
}

#[test]
fn app_state_handle_multiple_text_and_backspace() 
{
    let mut state = AppState::new();
    state.push_vec_user_input(vec!
    [
        (PageId::Page1, ButtonId::ButtonPurpleInputStartPage1),
        (PageId::Page1, ButtonId::ButtonRedInputStartPage1)
    ]);

    state.capturing_input = (true, Some(ButtonId::ButtonPurpleInputStartPage1));
    state.handle_text("hello".to_string());
    assert_eq!(state.vec_user_input[0].0, "hello");
    state.handle_backspace();
    assert_eq!(state.vec_user_input[0].0, "hell");

    state.capturing_input = (true, Some(ButtonId::ButtonRedInputStartPage1));
    state.handle_text("world".to_string());
    assert_eq!(state.vec_user_input[1].0, "world");
    state.handle_backspace();
    assert_eq!(state.vec_user_input[1].0, "worl");
}

#[test]
fn app_state_submit_input() 
{
    let mut state = AppState::new();
    state.capturing_input = (true, Some(ButtonId::ButtonPurpleInputStartPage1));
    state.submit_input();
    assert!(!state.capturing_input.0);
    assert_eq!(None, state.capturing_input.1);
}

#[test]
fn app_state_handle_action_switch_page() 
{
    let mut state = AppState::new();
    // Test switching to Page2
    button_action(&mut state, ButtonId::ButtonPage2);
    assert_eq!(state.current_page, PageId::Page2);
    // Test switching to Page1
    button_action(&mut state, ButtonId::ButtonPage1);
    assert_eq!(state.current_page, PageId::Page1);
    // Test subpage and back
    button_action(&mut state, ButtonId::ButtonSubPage);
    assert_eq!(state.current_page, PageId::Page2SubPage);
    button_action(&mut state, ButtonId::ButtonBack);
    assert_eq!(state.current_page, PageId::Page2);
}

#[test]
fn app_state_handle_action_starts_input_capture() 
{
    let mut state = AppState::new();
    let input_button = ButtonId::ButtonPurpleInputStartPage1;
    button_action(&mut state, input_button);
    assert!(state.capturing_input.0);
    assert_eq!(state.capturing_input.1, Some(input_button));
}

#[test]
fn app_state_page_at_none_when_no_button() 
{
    let mut state = AppState::new();
    state.current_page = PageId::Page2SubPage;

    // These coordinates are likely not on any button
    let button_on_position = state.page_button_at(-10000.0, -10000.0);
    // See If Button Is Being Returned None When There Is No Button Selected
    assert!(button_on_position.is_none());
}
