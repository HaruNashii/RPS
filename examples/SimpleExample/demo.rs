use std::{env, time::Duration};
use sdl3::{pixels::Color, rect::Rect};
use rust_page_system::
{
    Button,
    misc::center_elements::get_center,
    system::
    {
        input_handler::{InputEvent, InputHandler}, 
        page_system::Page, 
        state::AppState, 
        window::{create_window, get_monitor_refresh_rate, WINDOW_DEFAULT_SCALE}
    }, 
};

// To Be Ignored, Just An Setup To Configure The Build
use crate::build::setup_build;
mod build;



//==========================================================================================================================================================================
//=======================================================================# main function recommended setup #===============================================================
//==========================================================================================================================================================================
fn main() 
{
    // To Be Ignored, Just An Setup To Configure The Build
    setup_build();

    let (mut canvas, mut event_pump, texture_creator, ttf_context) = create_window(false);
    let input_handler = InputHandler;
    let mut app_state = AppState {current_page: (PageId::Page1 as usize, true), vec_user_input: Vec::new(), vec_user_input_string: Vec::new(), capturing_input: (false, None), window_size: WINDOW_DEFAULT_SCALE, all_pages: Vec::new() };
    populate_or_update_app_state(&mut app_state, false);

    let refresh_rate = get_monitor_refresh_rate();
    'running: loop 
    {
        std::thread::sleep(Duration::from_millis(1000 / refresh_rate));
        match input_handler.poll(&mut event_pump) 
        {
            InputEvent::Click(x, y)   => if let Some(button_id) = app_state.page_button_at(x, y) { button_action(&mut app_state, button_id); },
            InputEvent::Text(string)    => app_state.handle_text(string),
            InputEvent::Backspace               => app_state.handle_backspace(),
            InputEvent::Submit                  => app_state.submit_input(),
            InputEvent::Quit                    => break 'running,
            InputEvent::None                    => {}
        }
        populate_or_update_app_state(&mut app_state, true);
        app_state.render(&mut canvas, &texture_creator, &ttf_context);
    }
}



//==========================================================================================================================================================================
//===============================================================# can be a different file, like: buttons_actions.rs #======================================================
//==========================================================================================================================================================================
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
//====================================================================# can be a different file, like: style.rs #===========================================================
//==========================================================================================================================================================================
pub const BACKGROUND_COLOR: Color = Color::RGB(30, 30, 46);
pub const TEXT_COLOR: Color = Color::RGB(255, 255, 255);
pub const SUBTEXT_COLOR: Color = Color::RGB(186, 194, 222);
pub const PURPLE_COLOR: Color = Color::RGB(203, 166, 247);
pub const PINK_COLOR: Color = Color::RGB(243, 139, 168);
pub const ORANGE_COLOR: Color = Color::RGB(250, 179, 135);
pub const BLACK_COLOR: Color = Color::RGB(17, 17, 27);
pub const RED_COLOR: Color = Color::RGB(255, 0, 0);



//==========================================================================================================================================================================
//===============================================================# can be a different file, like: pages.rs #================================================================
//==========================================================================================================================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Defines The ID for your Pages
pub enum PageId 
{
    Persistent,
    Page1,
    Page1SubPage,
}
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

pub fn populate_or_update_app_state(app_state: &mut AppState, only_update: bool)
{
    if !only_update
    {
        //Populate Vec_Of_User_input With Page And Buttons That Receives User_Input
        app_state.push_vec_user_input(vec!
        [
            (PageId::Page1 as usize, ButtonId::ButtonPurpleInputStartPage1 as usize),
        ]);
    }
    
    app_state.populate_and_update_all_pages(vec!
    [
        //Persistent Page Needs To Always Be First
        persistent_page(),
        page_1(&app_state.vec_user_input_string),
        subpage_page1(),
    ]);
}

// Define Your Pages Here:
pub fn persistent_page() -> Page
{
    //===================== variables =========================
    let window_center = get_center((200, 75), WINDOW_DEFAULT_SCALE);

    //===================== rects =========================
    let all_rects = vec! [ (BLACK_COLOR, (Rect::new(0, 0, WINDOW_DEFAULT_SCALE.0, 100), 0)) ];

    //===================== buttons =========================
    let all_buttons = vec! [ Button { enabled: true, color: PINK_COLOR, rect: Rect::new(window_center.pos_x, 10, window_center.w, window_center.h), radius: 5, id: ButtonId::ButtonPage1 as usize }, ];

    //===================== texts =========================
    let all_text = vec! [ (17.0, (all_buttons[0].rect.x + 9, all_buttons[0].rect.y + 24), "Page 1".to_string(), TEXT_COLOR), ];

    //===================== images =========================
    let all_images = vec!
    [
        ((10, 10), (50, 50), format!("{}/.cache/page_system/example_1.jpg", env::home_dir().unwrap().display()))
    ];

    //===================== page creation =========================
    Page { has_persistant_page: false, id: PageId::Persistent as usize, background_color: None, rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_text), images: Some(all_images) }
}

pub fn page_1(user_input: &[String]) -> Page
{
    //===================== variables =========================
    let purple_button_data = get_center((600, 100), WINDOW_DEFAULT_SCALE);
    let subpage_button_data = get_center((235, 40), WINDOW_DEFAULT_SCALE);

    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(subpage_button_data.pos_x, 150, subpage_button_data.w, subpage_button_data.h), radius: 20, id: ButtonId::ButtonSubPage as usize },
        Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(purple_button_data.pos_x, purple_button_data.pos_y, purple_button_data.w, purple_button_data.h), radius: 5, id: ButtonId::ButtonPurpleInputStartPage1 as usize },
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "Go To subpage_page1".to_string(), TEXT_COLOR),
        (18.0, (all_buttons[1].rect.x + 75, all_buttons[1].rect.y - 25), "Click the Button To Start Getting Input".to_string(), SUBTEXT_COLOR),
        (25.0, (all_buttons[1].rect.x + 15, all_buttons[1].rect.y + 35), user_input[0].clone(), BLACK_COLOR),
    ];

    //===================== page creation =========================
    Page { has_persistant_page: true, id: PageId::Page1 as usize, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }
}

pub fn subpage_page1() -> Page
{
    //===================== buttons =========================
    let all_buttons = vec! [ Button { enabled: true, color: PINK_COLOR, rect: Rect::new(20, 20, 50, 40), radius: 0, id: ButtonId::ButtonBack as usize }];

    //===================== texts =========================
    let all_text = vec! [ (18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "<-".to_string(), TEXT_COLOR) ];

    //===================== page creation =========================
    Page { has_persistant_page: false, id: PageId::Page1SubPage as usize, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }
}
