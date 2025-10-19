use rust_page_system::{system::{page_system::PageData, state::AppState, window::WINDOW_DEFAULT_SCALE}, InputHandler};
use crate::setup_tests::
{
    button_action, page_1, page_2, persistent_elements1, persistent_elements2, populate_or_update_app_state, subpage_page2, ButtonId, PageId
};



#[cfg(test)]
mod setup_tests;



#[test]
fn app_initialization() 
{
    let state = AppState::<PageId, ButtonId>::new(PageId::Page1, true);
    let data = PageData::<PageId, ButtonId>::new();
    assert_eq!(state.current_page, (PageId::Page1, true));
    assert_eq!(data.vec_user_input.len(), 0);
    assert_eq!(data.vec_user_input_string.len(), 0);
    assert_eq!(state.capturing_input, (false, None));
    assert_eq!(state.window_size, WINDOW_DEFAULT_SCALE);
}

#[test]
pub fn app_state_define_persistent_elements()
{
    let mut data = PageData::<PageId, ButtonId>::new();
    let vec_of_persistent_elementss = vec!
    [
        persistent_elements1(),
        persistent_elements2()
    ];
    data.define_persistent_elements(vec_of_persistent_elementss.clone());
    assert_eq!(data.persistent_elements, vec_of_persistent_elementss);
}

#[test]
pub fn app_state_populate_and_update_all_pages()
{
    let mut data = PageData::<PageId, ButtonId>::new();
    let vec_to_populate = vec!
    [
        page_1(&data.vec_user_input_string),
        page_2(&data.vec_user_input_string),
        subpage_page2(),
    ];
    data.populate_and_update_all_pages(vec_to_populate.clone());
    assert_eq!(data.all_pages, vec_to_populate); 
}

#[test]
fn app_state_push_vec_user_input() 
{
    let mut data = PageData::<PageId, ButtonId>::new();
    data.push_vec_user_input(vec!
    [
        (PageId::Page1, ButtonId::ButtonPurpleInputStartPage1),
    ]);

    // Check The Len Of The Vec_User_Input
    assert_eq!(data.vec_user_input.len(), 1);
    // Check correct PageId and ButtonId
    assert_eq!(data.vec_user_input[0].0, PageId::Page1);
    assert_eq!(data.vec_user_input[0].1, ButtonId::ButtonPurpleInputStartPage1);
}

#[test]
fn app_state_handle_multiple_text_and_backspace() 
{
    let mut state = AppState::<PageId, ButtonId>::new(PageId::Page1, true);
    let mut data = PageData::<PageId, ButtonId>::new();
    let mut input_handler = InputHandler::<PageId, ButtonId>::new();

    populate_or_update_app_state(&mut data, false);

    // Try Pushing Input To Button Purple Page 1 And Removing One Letter
    state.current_page = (PageId::Page1, true);
    state.capturing_input = (true, Some(ButtonId::ButtonPurpleInputStartPage1));
    input_handler.handle_text("hello".to_string(), &mut state, &mut data);
    assert_eq!(data.vec_user_input_string[0], "hello");
    input_handler.handle_backspace(&mut state, &mut data);
    assert_eq!(data.vec_user_input_string[0], "hell");

    // Try Pushing Input To Button Red Page 1 And Removing One Letter
    state.current_page = (PageId::Page1, true);
    state.capturing_input = (true, Some(ButtonId::ButtonRedInputStartPage1 ));
    assert_eq!(data.vec_user_input_string[1], "");
    input_handler.handle_text("world".to_string(), &mut state, &mut data);
    assert_eq!(data.vec_user_input_string[1], "world");
    input_handler.handle_backspace(&mut state, &mut data);
    assert_eq!(data.vec_user_input_string[1], "worl");


    // Try Pushing Input To Button Purple Page 2 And Removing One Letter
    state.current_page = (PageId::Page2, true);
    state.capturing_input = (true, Some(ButtonId::ButtonPurpleInputStartPage2 ));
    input_handler.handle_text("test".to_string(), &mut state, &mut data);
    assert_eq!(data.vec_user_input_string[2], "test");
    input_handler.handle_backspace(&mut state, &mut data);
    assert_eq!(data.vec_user_input_string[2], "tes");
}

#[test]
fn app_state_submit_input() 
{
    let mut state = AppState::<PageId, ButtonId>::new(PageId::Page1, true);
    let mut input_handler = InputHandler::<PageId, ButtonId>::new();
    state.capturing_input = (true, Some(ButtonId::ButtonPurpleInputStartPage1 ));
    input_handler.submit_input(&mut state);
    assert!(!state.capturing_input.0);
    assert_eq!(None, state.capturing_input.1);
}

#[test]
fn app_state_handle_action_switch_page() 
{
    let mut state = AppState::<PageId, ButtonId>::new(PageId::Page1, true);
    // Test switching to Page2
    button_action(&mut state, ButtonId::ButtonPage2 );
    assert_eq!(state.current_page.0, PageId::Page2 );
    // Test switch to subpage
    button_action(&mut state, ButtonId::ButtonSubPage );
    assert_eq!(state.current_page.0, PageId::Page2SubPage );
    // Test Switch Back to Page2
    button_action(&mut state, ButtonId::ButtonBack );
    assert_eq!(state.current_page.0, PageId::Page2 );
}

#[test]
fn app_state_handle_action_starts_input_capture() 
{
    let mut state = AppState::<PageId, ButtonId>::new(PageId::Page1, true);
    let mut data = PageData::<PageId, ButtonId>::new();

    //Populate Vec_Of_User_input With Page And Buttons That Receives User_Input
    data.push_vec_user_input(vec!
    [
        (PageId::Page1 , ButtonId::ButtonPurpleInputStartPage1),
        (PageId::Page1 , ButtonId::ButtonRedInputStartPage1),
        (PageId::Page2 , ButtonId::ButtonPurpleInputStartPage2),
    ]);


    //Test Starting User Input In Purple Button
    state.current_page.0 = PageId::Page1 ;
    let input_button = ButtonId::ButtonPurpleInputStartPage1 ;
    button_action(&mut state, input_button);
    assert!(state.capturing_input.0);
    assert_eq!(state.capturing_input.1, Some(input_button));
    state.capturing_input.0 = false;
    state.capturing_input.1 = None;

    //Test Start User Input In Red Button
    let input_button = ButtonId::ButtonRedInputStartPage1 ;
    button_action(&mut state, input_button);
    assert!(state.capturing_input.0);
    assert_eq!(state.capturing_input.1, Some(input_button));
    state.capturing_input.0 = false;
    state.capturing_input.1 = None;

    //Test Start User Input In Purple Button In Page2
    state.current_page.0 = PageId::Page2 ;
    let input_button = ButtonId::ButtonPurpleInputStartPage2 ;
    button_action(&mut state, input_button);
    assert!(state.capturing_input.0);
    assert_eq!(state.capturing_input.1, Some(input_button));
}

#[test]
fn app_state_page_at_none_when_no_button() 
{
    let mut state = AppState::<PageId, ButtonId>::new(PageId::Page2SubPage, false);
    let data = PageData::<PageId, ButtonId>::new();

    // These coordinates are likely not on any button
    let button_on_position = data.page_button_at(&mut state, -10000.0, -10000.0);
    // See If Button Is Being Returned None When There Is No Button Selected
    assert!(button_on_position.is_none());
}
