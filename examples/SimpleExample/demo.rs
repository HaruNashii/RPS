use std::{env, time::Duration};
use sdl3::{pixels::Color, rect::Rect};
use rust_page_system::
{
    Button,
    Renderer,
    misc::center_elements::get_center, 
    system::
    {
        input_handler::{InputEvent, InputHandler}, 
        page_system::{Page, PageData}, 
        state::AppState, 
        window::{create_window, get_monitor_refresh_rate, WindowConfig}
    }
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

    let window_config = WindowConfig
    {
        window_title: "SimpleExample".to_string(),
        icon: (false, None),
        // Recommended to start with 16:9 aspect ratio
        start_window_size: (800, 450),
        // Recommended to have minimum size with 16:9 aspect ratio
        window_minimum_size: (800, 450),
        resizable: true,
        centered: true,
        hint_sdl3_vsync: true
    };
    let (mut canvas, mut event_pump, texture_creator, ttf_context) = create_window(window_config);
    let mut input_handler = InputHandler::new();
    let mut app_state = AppState::new(PageId::Page1, true);
    let mut page_data = PageData::new();
    let mut renderer = Renderer::new(&mut canvas, &texture_creator, &ttf_context);

    populate_page_data(&mut page_data);

    let refresh_rate = get_monitor_refresh_rate();
    'running: loop 
    {
        std::thread::sleep(Duration::from_millis(1000 / refresh_rate));
        match input_handler.poll(&mut event_pump) 
        {
            InputEvent::Click(x, y)   => if let Some(button_id) = page_data.page_button_at(&app_state, x, y) { button_action(&mut app_state, &button_id); },
            InputEvent::Text(string)    => input_handler.handle_text(string, &mut app_state, &mut page_data),
            InputEvent::Backspace               => input_handler.handle_backspace(&mut app_state, &mut page_data),
            InputEvent::Submit                  => input_handler.submit_input(&mut app_state),
            InputEvent::Quit                    => break 'running,
            InputEvent::None                    => {}
        }
        app_state.update_window_size(renderer.canvas.window().size());
        update_page_data(&mut page_data);
        renderer.render(&app_state, &page_data);
    }
}



//==========================================================================================================================================================================
//===============================================================# can be a different file, like: buttons_actions.rs #======================================================
//==========================================================================================================================================================================
pub fn button_action(app_state: &mut AppState<PageId, ButtonId>, button_id: &ButtonId) 
{
    if !app_state.capturing_input.0
    {
        if &ButtonId::ButtonPage1    == button_id {app_state.current_page = (PageId::Page1,        true);    return};
        if &ButtonId::ButtonSubPage  == button_id {app_state.current_page = (PageId::Page1SubPage, true);    return};
        if &ButtonId::ButtonBack     == button_id {app_state.current_page = (PageId::Page1,        true);    return};
        // Non Handle Buttons Will Be Considered User Input Buttons
        app_state.capturing_input = (true, Some(*button_id));
    }
}



//==========================================================================================================================================================================
//===============================================================# can be a different file, like: setup_page_data.rs #======================================================
//==========================================================================================================================================================================
pub fn populate_page_data(page_data: &mut PageData<PageId, ButtonId>)
{
    //Populate Vec_Of_User_input With Page And Buttons That Receives User_Input
    page_data.push_vec_user_input(vec!
    [
        (PageId::Page1, ButtonId::ButtonPurpleInputStartPage1),
    ]);
    //Populate Persistent Elements with your defined persistent elements, (If your Persistent
    //Elements have runtime changing elements, like: Userinput, you need to place this definition inside an loop)
    page_data.define_persistent_elements(vec! 
    [
        persistent_elements(),
    ]);
    
}
pub fn update_page_data(page_data: &mut PageData<PageId, ButtonId>)
{
    //Populate PageData allpages vector
    page_data.populate_and_update_all_pages(vec!
    [
        page_1(&page_data.vec_user_input_string),
        subpage_page1(),
    ]);
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
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[repr(usize)]
/// Defines The ID for your Buttons
pub enum ButtonId 
{
    ButtonPage1,
    ButtonPurpleInputStartPage1,
    ButtonSubPage,
    ButtonBack,
}



// Define Your Pages Here:
pub fn persistent_elements() -> Page<PageId, ButtonId>
{
    //===================== rects =========================
    let all_rects = vec! [ (BLACK_COLOR, (Rect::new(0, 0, 1920, 100), 0)) ];

    //===================== texts =========================
    let all_text = vec! [ (17.0, (825, 34), "This Is A Persistent Element".to_string(), TEXT_COLOR), ];

    //===================== images =========================
    let all_images = vec!
    [
        ((10, 10), (50, 50), format!("{}/.cache/page_system/example_1.jpg", env::home_dir().unwrap().display()))
    ];

    //===================== page creation =========================
    Page { has_persistent_elements: (false, None), id: PageId::Persistent, background_color: None, rects: Some(all_rects), buttons: None, texts: Some(all_text), images: Some(all_images) }
}

pub fn page_1(user_input: &[String]) -> Page<PageId, ButtonId>
{
    //===================== variables =========================
    let purple_button_data = get_center((600, 100), (1920, 1080));
    let subpage_button_data = get_center((235, 40), (1920, 1080));

    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(subpage_button_data.pos_x, 150, subpage_button_data.w, subpage_button_data.h), radius: 20, id: ButtonId::ButtonSubPage},
        Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(purple_button_data.pos_x, purple_button_data.pos_y, purple_button_data.w, purple_button_data.h), radius: 5, id: ButtonId::ButtonPurpleInputStartPage1},
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "Go To subpage_page1".to_string(), TEXT_COLOR),
        (18.0, (all_buttons[1].rect.x + 75, all_buttons[1].rect.y - 25), "Click the Button To Start Getting Input".to_string(), SUBTEXT_COLOR),
        (25.0, (all_buttons[1].rect.x + 15, all_buttons[1].rect.y + 35), user_input[0].to_string(), BLACK_COLOR),
    ];

    //===================== page creation =========================
    Page { has_persistent_elements: (true, Some(vec![PageId::Persistent])), id: PageId::Page1, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }

}

pub fn subpage_page1() -> Page<PageId,ButtonId>
{
    //===================== buttons =========================
    let all_buttons = vec! [ Button { enabled: true, color: PINK_COLOR, rect: Rect::new(20, 20, 50, 40), radius: 0, id: ButtonId::ButtonBack}];

    //===================== texts =========================
    let all_text = vec! [ (18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "<-".to_string(), TEXT_COLOR) ];

    //===================== page creation =========================
    Page { has_persistent_elements: (false, None), id: PageId::Page1SubPage, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }
}
