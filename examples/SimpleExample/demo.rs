use rust_page_system::{
    Button, Renderer,
    misc::{center_elements::get_center, vec::GetOrCreate},
    system::{
        input_handler::InputHandler,
        page_system::{Page, PageData, PersistentElements},
        renderer::RendererConfig,
        state::AppState,
        window::{WindowConfig, create_window, get_monitor_refresh_rate}
    }
};
use sdl3::{pixels::Color, rect::Rect, sys::render::SDL_LOGICAL_PRESENTATION_STRETCH};
use std::{env, rc::Rc, time::Duration};

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
        icon: (None, None),
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
    let mut input_handler = InputHandler::new(true);
    let mut app_state = AppState::new(PageId::Page1, window_modules.canvas.window().size(), window_modules.stretch_mode_status);
    let mut page_data = PageData::new(&app_state);
    let renderer_config = RendererConfig { canvas: window_modules.canvas, texture_creator: &window_modules.texture_creator, ttf_context: &window_modules.ttf_context, font_path: &window_modules.font_path, decrease_color_when_selected: Some((25, 25, 25)), selection_color: Some((0, 0, 200, 125)), assets_dir: None };
    let mut renderer = Renderer::new(renderer_config);

    populate_page_data(&mut page_data);

    // Wrap the button_action function in a mutable closure so it can capture
    // additional context if needed. Passing a closure here allows the
    // button handler API to accept additional arguments beyond the default.
    let mut button_action_closure = |app_state: &mut AppState<PageId, ButtonId>, button_id: &ButtonId, page_data: &mut PageData<PageId, ButtonId>| button_action(app_state, button_id, page_data);

    loop
    {
        println!("{:?}", page_data.persistent_elements_to_render.is_some());
        //using 900 / your_refresh_rate to a very crispy experience
        std::thread::sleep(Duration::from_millis(900 / get_monitor_refresh_rate()));
        app_state.update_window_size(renderer.canvas.window().size().0, renderer.canvas.window().size().1);
        input_handler.handle_input(&mut window_modules.event_pump, &mut window_modules.clipboard_system, &mut page_data, &mut app_state, &mut button_action_closure);
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
        if &ButtonId::ButtonPage1 == button_id
        {
            app_state.change_current_page(app_data, PageId::Page1, button_id);
            return;
        };
        if &ButtonId::ButtonSubPage == button_id
        {
            app_state.change_current_page(app_data, PageId::Page1SubPage, button_id);
            return;
        };
        if &ButtonId::ButtonBack == button_id
        {
            println!("button back clicked");
            app_state.change_current_page(app_data, PageId::Page1, button_id);
            return;
        };
        // Non Handle Buttons Will Be Considered User Input Buttons
        app_state.capturing_input = (true, Some(*button_id));
    }
}

//==========================================================================================================================================================================
//===============================================================# can be a different file, like: setup_page_data.rs #======================================================
//==========================================================================================================================================================================
pub fn populate_page_data(page_data: &mut PageData<PageId, ButtonId>)
{
    page_data.push_page_link(Some(vec![(PageId::Page1SubPage, Rc::new(subpage_page1))]), Some(vec![(PageId::Page1, Rc::new(|input: &mut Vec<String>| page_1(input, 13)))]));
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
    Page1SubPage
}
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[repr(usize)]
/// Defines The ID for your Buttons
pub enum ButtonId
{
    ButtonPage1,
    ButtonPurpleInputStartPage1,
    ButtonSubPage,
    ButtonBack
}

// Define Your Pages Here:
pub fn persistent_elements(_string: String) -> PersistentElements<PageId, ButtonId>
{
    //"persistent_elements now can also receive extra args without affecting the functionality of the app"
    // like the parsed = _string
    //===================== rects =========================
    let all_rects = vec![(BLACK_COLOR, (Rect::new(0, 0, 1920, 100), 0))];

    //===================== texts =========================
    let all_text = vec![(17.0, (825, 34), "This Is A Persistent Element".to_string(), TEXT_COLOR)];

    //===================== images =========================
    let all_images = vec![((10, 10), (50, 50), format!("{}/.cache/page_system/example_1.jpg", env::home_dir().unwrap().display()))];

    //===================== page creation =========================
    PersistentElements { id: PageId::Persistent, background_color: None, rects: Some(all_rects), buttons: None, texts: Some(all_text), images: Some(all_images) }
}

pub fn page_1(user_input: &mut Vec<String>, _int: i32) -> Page<PageId, ButtonId>
{
    //"pages now can also receive extra args without affecting the functionality of the app"
    // like the parsed = _int
    //===================== variables =========================
    let purple_button_data = get_center((600, 100), (1920, 1080));
    let subpage_button_data = get_center((235, 40), (1920, 1080));

    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(subpage_button_data.pos_x, 150, subpage_button_data.w, subpage_button_data.h), radius: 20, id: ButtonId::ButtonSubPage, has_transition: None }, Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(purple_button_data.pos_x, purple_button_data.pos_y, purple_button_data.w, purple_button_data.h), radius: 5, id: ButtonId::ButtonPurpleInputStartPage1, has_transition: None }];

    //===================== texts =========================
    let all_text = vec![(18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "Go To subpage_page1".to_string(), TEXT_COLOR), (18.0, (all_buttons[1].rect.x + 75, all_buttons[1].rect.y - 25), "Click the Button To Start Getting Input".to_string(), SUBTEXT_COLOR), (25.0, (all_buttons[1].rect.x + 15, all_buttons[1].rect.y + 35), user_input.get_or_create(0), BLACK_COLOR)];

    //===================== page creation =========================
    Page { has_userinput: Some(vec![(PageId::Page1, ButtonId::ButtonPurpleInputStartPage1)]), has_persistent_elements: Some(vec![(PageId::Persistent, Rc::new(|| persistent_elements("a".to_string())))]), id: PageId::Page1, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }
}

pub fn subpage_page1() -> Page<PageId, ButtonId>
{
    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: PINK_COLOR, rect: Rect::new(200, 0, 50, 150), radius: 0, id: ButtonId::ButtonBack, has_transition: None }];

    //===================== texts =========================
    let all_text = vec![(18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "<-".to_string(), TEXT_COLOR)];

    //===================== page creation =========================
    Page { has_userinput: None, has_persistent_elements: None, id: PageId::Page1SubPage, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }
}
