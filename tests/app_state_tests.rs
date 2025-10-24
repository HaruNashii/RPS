use rust_page_system::{system::{page_system::PageData, state::AppState, window::WINDOW_DEFAULT_SCALE}, InputHandler};
use crate::setup_tests::{button_action, populate_page_data, ButtonId, PageId};



#[cfg(test)]
mod setup_tests;



#[test]
fn app_initialization() 
{
    let state = AppState::<PageId, ButtonId>::new(PageId::Page1);
    let data = PageData::new(&state);
    assert_eq!(state.current_page, PageId::Page1);
    assert_eq!(data.vec_user_input.len(), 0);
    assert_eq!(data.vec_user_input_string.len(), 0);
    assert_eq!(state.capturing_input, (false, None));
    assert_eq!(state.window_size, WINDOW_DEFAULT_SCALE);
}

// Current Commented Bc Of Continuos refactory that breaks this test
#[test]
fn app_state_handle_multiple_text_and_backspace() 
{
    let mut state = AppState::new(PageId::Page1);
    let mut data = PageData::new(&state);
    let mut input_handler = InputHandler::<PageId, ButtonId>::new(true);

    populate_page_data(&mut data);
    data.push_vec_user_input_per_vec();


    // Try Pushing Input To Button Purple Page 1 And Removing One Letter
    state.capturing_input = (true, Some(ButtonId::ButtonPurpleInputStartPage1));
    input_handler.handle_text("hello".to_string(), &mut state, &mut data, None, 0);
    assert_eq!(data.vec_user_input_string[0], "hello");

    input_handler.handle_backspace(&mut state, &mut data);
    assert_eq!(data.vec_user_input_string[0], "hell");

    // Try Pushing Input To Button Red Page 1 And Removing One Letter
    state.current_page = PageId::Page1;
    state.capturing_input = (true, Some(ButtonId::ButtonRedInputStartPage1 ));
    assert_eq!(data.vec_user_input_string[1], "");
    input_handler.handle_text("world".to_string(), &mut state, &mut data, None, 0);
    assert_eq!(data.vec_user_input_string[1], "world");
    input_handler.handle_backspace(&mut state, &mut data);
    assert_eq!(data.vec_user_input_string[1], "worl");


    // Try Pushing Input To Button Purple Page 2 And Removing One Letter
    state.current_page = PageId::Page2;
    state.capturing_input = (true, Some(ButtonId::ButtonPurpleInputStartPage2 ));
    input_handler.handle_text("test".to_string(), &mut state, &mut data, None, 0);
    assert_eq!(data.vec_user_input_string[2], "test");
    input_handler.handle_backspace(&mut state, &mut data);
    assert_eq!(data.vec_user_input_string[2], "tes");
}

#[test]
fn app_state_submit_input() 
{
    let mut state = AppState::new(PageId::Page1);
    let mut input_handler = InputHandler::<PageId, ButtonId>::new(true);
    state.capturing_input = (true, Some(ButtonId::ButtonPurpleInputStartPage1 ));
    input_handler.submit_input(&mut state);
    assert!(!state.capturing_input.0);
    assert_eq!(None, state.capturing_input.1);
}

#[test]
fn app_state_handle_action_switch_page() 
{
    let mut state = AppState::new(PageId::Page1);
    let mut data = PageData::new(&state);
    // Test switching to Page2
    button_action(&mut state, &ButtonId::ButtonPage2, &mut data);
    assert_eq!(state.current_page, PageId::Page2 );
    // Test switch to subpage
    button_action(&mut state, &ButtonId::ButtonSubPage, &mut data);
    assert_eq!(state.current_page, PageId::Page2SubPage );
    // Test Switch Back to Page2
    button_action(&mut state, &ButtonId::ButtonBack, &mut data);
    assert_eq!(state.current_page, PageId::Page2 );
}

#[test]
fn app_state_handle_action_starts_input_capture() 
{
    let mut state = AppState::new(PageId::Page1);
    let mut data = PageData::new(&state);

    populate_page_data(&mut data);

    //Test Starting User Input In Purple Button
    state.current_page = PageId::Page1 ;
    let input_button = ButtonId::ButtonPurpleInputStartPage1 ;
    button_action(&mut state, &input_button, &mut data);
    assert!(state.capturing_input.0);
    assert_eq!(state.capturing_input.1, Some(input_button));
    state.capturing_input.0 = false;
    state.capturing_input.1 = None;

    //Test Start User Input In Red Button
    let input_button = ButtonId::ButtonRedInputStartPage1 ;
    button_action(&mut state, &input_button, &mut data);
    assert!(state.capturing_input.0);
    assert_eq!(state.capturing_input.1, Some(input_button));
    state.capturing_input.0 = false;
    state.capturing_input.1 = None;

    //Test Start User Input In Purple Button In Page2
    state.current_page = PageId::Page2 ;
    let input_button = ButtonId::ButtonPurpleInputStartPage2 ;
    button_action(&mut state, &input_button, &mut data);
    assert!(state.capturing_input.0);
    assert_eq!(state.capturing_input.1, Some(input_button));
}

#[test]
fn app_state_page_at_none_when_no_button() 
{
    let mut state = AppState::<PageId, ButtonId>::new(PageId::Page2SubPage);
    let data = PageData::new(&state);

    // These coordinates are likely not on any button
    let button_on_position = data.page_button_at(&mut state, -10000.0, -10000.0);
    // See If Button Is Being Returned None When There Is No Button Selected
    assert!(button_on_position.is_none());
}
