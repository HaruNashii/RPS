use std::{env, time::Duration};
use sdl3::{pixels::Color, rect::Rect, sys::render::SDL_LOGICAL_PRESENTATION_STRETCH};
use rust_page_system::
{
    misc::{center_elements::get_center, vec::GetOrCreate}, system::
    {
        input_handler::InputHandler, page_system::{Page, PageData, PersistentElements}, state::AppState, window::{create_window, get_monitor_refresh_rate, WindowConfig}
    }, Button, Renderer
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
        // By Default SDL_LOGICAL_PRESENTATION_STRETCH Is Set, Only Setting It Here For Demonstration Purpose 
        different_sdl_presentation_mode: Some(SDL_LOGICAL_PRESENTATION_STRETCH), 
        font: ("JetBrainsMono".to_string(), Some("Bold".to_string()))
    };
    let mut window_modules = create_window(window_config);
    //bool is reffered to the rollback pages system, with "Mouse side buttons" or ("Alt" + "Arrows Keys") | (false = Page Rollback On), (true = Page Rollback Off)
    let mut input_handler = InputHandler::new(false);
    let mut app_state = AppState::new(PageId::Page1, window_modules.canvas.window().size());
    let mut page_data = PageData::new(&app_state);
    let mut renderer = Renderer::new(window_modules.canvas, &window_modules.texture_creator, &window_modules.ttf_context, &window_modules.font_path, Some((25, 25, 25)), Some((0, 0, 200, 125)));

    populate_page_data(&mut page_data);

    loop 
    {
        //using 900 / your_refresh_rate to a very crispy experience
        std::thread::sleep(Duration::from_millis(900 / get_monitor_refresh_rate()));
        app_state.update_window_size(renderer.canvas.window().size().0, renderer.canvas.window().size().1);
        input_handler.handle_input(&mut window_modules.event_pump, &mut window_modules.clipboard_system, &mut page_data, &mut app_state, button_action);
        page_data.create_current_page(&mut app_state);
        renderer.render(&page_data, &mut app_state, &input_handler);
    }
}




//==========================================================================================================================================================================
//===============================================================# can be a different file, like: buttons_actions.rs #======================================================
//==========================================================================================================================================================================
pub fn button_action(app_state: &mut AppState<PageId, ButtonId>, button_id: &ButtonId, app_data: &mut PageData<PageId, ButtonId>) 
{
    if !app_state.capturing_input.0
    {
        if &ButtonId::ButtonPage1    == button_id {app_state.change_current_page(app_data, PageId::Page1); return};
        if &ButtonId::ButtonSubPage  == button_id {app_state.change_current_page(app_data, PageId::Page1SubPage); return};
        if &ButtonId::ButtonBack     == button_id {app_state.change_current_page(app_data, PageId::Page1); return};
        // Non Handle Buttons Will Be Considered User Input Buttons
        app_state.capturing_input = (true, Some(*button_id));
    }
}





//==========================================================================================================================================================================
//===============================================================# can be a different file, like: setup_page_data.rs #======================================================
//==========================================================================================================================================================================
pub fn populate_page_data(page_data: &mut PageData<PageId, ButtonId>)
{
    page_data.push_page_link
    (
        Some(vec![(PageId::Page1SubPage, subpage_page1)]),
        Some(vec![(PageId::Page1, page_1)])
    );
}



//==========================================================================================================================================================================
//====================================================================# can be a different file, like: style.rs (or not even exist) #=======================================
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
pub fn persistent_elements() -> PersistentElements<PageId, ButtonId>
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
    PersistentElements { id: PageId::Persistent, background_color: None, rects: Some(all_rects), buttons: None, texts: Some(all_text), images: Some(all_images) }
}

pub fn page_1(user_input: &mut Vec<String>) -> Page<PageId, ButtonId>
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
        (25.0, (all_buttons[1].rect.x + 15, all_buttons[1].rect.y + 35), user_input.get_or_create(0), BLACK_COLOR),
    ];

    //===================== page creation =========================
    Page { has_userinput: Some(vec![(PageId::Page1, ButtonId::ButtonPurpleInputStartPage1)]), has_persistent_elements: Some(vec![(PageId::Persistent, persistent_elements)]), has_transition: None, id: PageId::Page1, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }

}

pub fn subpage_page1() -> Page<PageId,ButtonId>
{
    //===================== buttons =========================
    let all_buttons = vec! [ Button { enabled: true, color: PINK_COLOR, rect: Rect::new(20, 20, 50, 40), radius: 0, id: ButtonId::ButtonBack}];

    //===================== texts =========================
    let all_text = vec! [ (18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "<-".to_string(), TEXT_COLOR) ];

    //===================== page creation =========================
    Page { has_userinput: None, has_persistent_elements: None, has_transition: None, id: PageId::Page1SubPage, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }
}
