use rust_page_system::{system::state::AppState, Page, Button, get_center};
use sdl3::{pixels::Color, rect::Rect};


pub const WINDOW_DEFAULT_SCALE: (u32, u32) = (1920, 1080);



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
//===============================================================# can be a different file, like: buttons_actions.rs #======================================================
//==========================================================================================================================================================================
#[cfg(test)]
pub fn button_action(app_state: &mut AppState, button_id: usize) 
{
    if !app_state.capturing_input.0
    {
        if ButtonId::ButtonPage1 as usize   == button_id {app_state.current_page.0 = PageId::Page1 as usize; app_state.current_page.1 = true; return};
        if ButtonId::ButtonPage2 as usize   == button_id {app_state.current_page.0 = PageId::Page2 as usize; app_state.current_page.1 = true; return};
        if ButtonId::ButtonSubPage as usize == button_id {app_state.current_page.0 = PageId::Page2SubPage as usize; return};
        if ButtonId::ButtonBack as usize    == button_id {app_state.current_page.0 = PageId::Page2 as usize; app_state.current_page.1 = true; return};
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
    Persistent1,
    Persistent2,
    Page1,
    Page2,
    Page2SubPage,
}
#[cfg(test)]
#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(usize)]
/// Defines The ID for your Buttons
pub enum ButtonId 
{
    ButtonPage1,
    ButtonPage2,
    ButtonSubPage,
    ButtonBack,
    ButtonPurpleInputStartPage1,
    ButtonRedInputStartPage1,
    ButtonPurpleInputStartPage2,
}

pub fn persistent_elements1() -> Page
{
    //===================== variables =========================
    let padding_x = 200;
    let window_center = get_center((200, 75), WINDOW_DEFAULT_SCALE);

    //===================== rects =========================
    let all_rects = vec!
    [
        (BLACK_COLOR, (Rect::new(0, 0, WINDOW_DEFAULT_SCALE.0, 100), 0))
    ];

    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: PINK_COLOR, rect: Rect::new(window_center.pos_x - padding_x, 10, window_center.w, window_center.h), radius: 5, id: ButtonId::ButtonPage1 as usize},
        Button { enabled: true, color: PINK_COLOR, rect: Rect::new(window_center.pos_x + padding_x, 10, window_center.w, window_center.h), radius: 5, id: ButtonId::ButtonPage2 as usize}
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        //page_1 button text
        (17.0, (all_buttons[0].rect.x + 9, all_buttons[0].rect.y + 24), "Page 1".to_string(), TEXT_COLOR),
        //page_2 button text
        (17.0, (all_buttons[1].rect.x + 9, all_buttons[1].rect.y + 24), "Page 2".to_string(), TEXT_COLOR),
    ];

    //===================== page creation =========================
    Page { has_persistent_elements: (false, None), id: PageId::Persistent1 as usize, background_color: None, rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_text), images: None }
}

pub fn persistent_elements2() -> Page 
{
    //===================== rects =========================
    let all_rects = vec!
    [
        (BLACK_COLOR, (Rect::new(0, 900, WINDOW_DEFAULT_SCALE.0, 999), 0))
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (17.0, (750, all_rects[0].1.0.y + 45), "This Rectangle Is From A Persistent Page".to_string(), TEXT_COLOR),
    ];

    //===================== page creation =========================
    Page { has_persistent_elements: (false, None), id: PageId::Persistent2 as usize, background_color: None, rects: Some(all_rects), buttons: None, texts: Some(all_text), images: None }
}

pub fn page_1(_: &[String]) -> Page
{
    //===================== variables =========================
    let padding_y = 20;
    let red_rect_data = get_center((200, 200), WINDOW_DEFAULT_SCALE);
    let orange_rect_data = get_center((800, 200), WINDOW_DEFAULT_SCALE);
    let purple_button_data = get_center((600, 100), WINDOW_DEFAULT_SCALE);

    //===================== rects =========================
    let all_rects = vec!
    [
        (RED_COLOR, (Rect::new(red_rect_data.pos_x, red_rect_data.pos_y + (orange_rect_data.h as i32 + padding_y), red_rect_data.w, red_rect_data.h), 100)),
        (ORANGE_COLOR, (Rect::new(orange_rect_data.pos_x, orange_rect_data.pos_y, orange_rect_data.w, orange_rect_data.h), 0))
    ];

    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(purple_button_data.pos_x, purple_button_data.pos_y - (orange_rect_data.h as i32 - padding_y),       purple_button_data.w, purple_button_data.h), radius: 5, id: ButtonId::ButtonPurpleInputStartPage1 as usize},
        Button { enabled: true, color: RED_COLOR,    rect: Rect::new(purple_button_data.pos_x, all_rects[0].1.0.y + all_rects[0].1.0.h + padding_y, purple_button_data.w, purple_button_data.h), radius: 20, id: ButtonId::ButtonRedInputStartPage1 as usize }
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (18.0, (all_rects[1].1.0.x + 165, all_rects[1].1.0.y + 86), "Random Orange Rectangle, Because I Can :)".to_string(), SUBTEXT_COLOR), 
        (18.0, (all_buttons[0].rect.x + 75, all_buttons[0].rect.y - 25), "Click the Button To Start Getting Input".to_string(), SUBTEXT_COLOR),
        //(25.0, (all_buttons[0].rect.x + 15, all_buttons[0].rect.y + 35), user_input[0].clone(), BLACK_COLOR),
        //(25.0, (all_buttons[1].rect.x + 15, all_buttons[1].rect.y + 35), user_input[1].clone(), BLACK_COLOR)
    ];

    //===================== page creation =========================
    Page { has_persistent_elements: (true, Some(vec![PageId::Persistent1 as usize])), id: PageId::Page1 as usize, background_color: Some(BACKGROUND_COLOR), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_text), images: None }
}

pub fn page_2(_: &[String]) -> Page
{
    //===================== variables =========================
    let get_input_button_data = get_center((500, 100), WINDOW_DEFAULT_SCALE);

    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(100, 150, 235, 40), radius: 20, id: ButtonId::ButtonSubPage as usize},
        Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(get_input_button_data.pos_x, get_input_button_data.pos_y, get_input_button_data.w as u32, get_input_button_data.h as u32), radius: 20, id: ButtonId::ButtonPurpleInputStartPage2 as usize }
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "Go To subpage_page2".to_string(), TEXT_COLOR),
        //(18.0, (all_buttons[1].rect.x + 10, all_buttons[1].rect.y + 7), user_input[2].clone(), TEXT_COLOR)
    ];

    //===================== page creation =========================
    Page { has_persistent_elements: (true, Some(vec![PageId::Persistent1 as usize, PageId::Persistent2 as usize])), id: PageId::Page2 as usize, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }
}

pub fn subpage_page2() -> Page
{
    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: PINK_COLOR, rect: Rect::new(20, 20, 50, 40), radius: 0, id: ButtonId::ButtonBack as usize}
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (18.0, (950, 400), "Random Text, Because I Can :)".to_string(), SUBTEXT_COLOR),
        (18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "<-".to_string(), TEXT_COLOR)
    ];

    //===================== page creation =========================
    Page { has_persistent_elements: (true, Some(vec![PageId::Persistent2 as usize])), id: PageId::Page2SubPage as usize, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }
}
