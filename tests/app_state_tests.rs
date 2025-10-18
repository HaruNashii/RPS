use rust_page_system::system::{state::AppState, window::WINDOW_DEFAULT_SCALE};
use crate::setup_tests::
{
    button_action, 
    page_1, 
    page_2, 
    persistent_page1, 
    persistent_page2, 
    subpage_page2, 
    ButtonId, 
    PageId
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
pub fn app_state_define_persistent_pages()
{
    let mut state = AppState::new();
    let vec_of_persistent_pages = vec!
    [
        persistent_page1(),
        persistent_page2()
    ];
    state.define_persistent_pages(vec_of_persistent_pages.clone());
    assert_eq!(state.persistent_page, vec_of_persistent_pages);
}

#[test]
pub fn app_state_populate_and_update_all_pages()
{
    let mut state = AppState::new();
    let vec_to_populate = vec!
    [
        page_1(&state.vec_user_input_string),
        page_2(&state.vec_user_input_string),
        subpage_page2(),
    ];
    state.populate_and_update_all_pages(vec_to_populate.clone());
    assert_eq!(state.all_pages, vec_to_populate); 
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
    assert_eq!(state.vec_user_input[0].0, PageId::Page1 as usize);
    assert_eq!(state.vec_user_input[0].1, ButtonId::ButtonPurpleInputStartPage1 as usize);
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
    assert_eq!(state.vec_user_input_string[0], "hello");
    state.handle_backspace();
    assert_eq!(state.vec_user_input_string[0], "hell");
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
    // Test switching to Page2
    button_action(&mut state, ButtonId::ButtonPage2 as usize);
    assert_eq!(state.current_page.0, PageId::Page2 as usize);
    // Test switch to subpage
    button_action(&mut state, ButtonId::ButtonSubPage as usize);
    assert_eq!(state.current_page.0, PageId::Page2SubPage as usize);
    // Test Switch Back to Page2
    button_action(&mut state, ButtonId::ButtonBack as usize);
    assert_eq!(state.current_page.0, PageId::Page2 as usize);
}

#[test]
fn app_state_handle_action_starts_input_capture() 
{
    let mut state = AppState::new();

    //Populate Vec_Of_User_input With Page And Buttons That Receives User_Input
    state.push_vec_user_input(vec!
    [
        (PageId::Page1 as usize, ButtonId::ButtonPurpleInputStartPage1 as usize),
        (PageId::Page1 as usize, ButtonId::ButtonRedInputStartPage1 as usize),
        (PageId::Page2 as usize, ButtonId::ButtonPurpleInputStartPage2 as usize),
    ]);


    //Test Starting User Input In Purple Button
    state.current_page.0 = PageId::Page1 as usize;
    let input_button = ButtonId::ButtonPurpleInputStartPage1 as usize;
    button_action(&mut state, input_button);
    assert!(state.capturing_input.0);
    assert_eq!(state.capturing_input.1, Some(input_button));
    state.capturing_input.0 = false;
    state.capturing_input.1 = None;

    //Test Start User Input In Red Button
    let input_button = ButtonId::ButtonRedInputStartPage1 as usize;
    button_action(&mut state, input_button);
    assert!(state.capturing_input.0);
    assert_eq!(state.capturing_input.1, Some(input_button));
    state.capturing_input.0 = false;
    state.capturing_input.1 = None;

    //Test Start User Input In Page2
    state.current_page.0 = PageId::Page2 as usize;
    let input_button = ButtonId::ButtonPurpleInputStartPage2 as usize;
    button_action(&mut state, input_button);
    assert!(state.capturing_input.0);
    assert_eq!(state.capturing_input.1, Some(input_button));
}

#[test]
fn app_state_page_at_none_when_no_button() 
{
    let mut state = AppState::new();
    state.current_page.0 = PageId::Page2SubPage as usize;
    // These coordinates are likely not on any button
    let button_on_position = state.page_button_at(-10000.0, -10000.0);
    // See If Button Is Being Returned None When There Is No Button Selected
    assert!(button_on_position.is_none());
}
