use crate::ui::pages::ButtonId;
use crate::{actions::buttons_actions::button_action, system::setup_page_data::populate_page_data, ui::pages::PageId};
use rust_page_system::{
    Renderer,
    system::{
        input_handler::InputHandler,
        page_system::PageData,
        renderer::RendererConfig,
        state::AppState,
        window::{WindowConfig, create_window, get_monitor_refresh_rate}
    }
};
use sdl3::sys::render::SDL_LOGICAL_PRESENTATION_STRETCH;
use std::time::Duration;

// To Be Ignored, Just An Setup To Configure Some Assets
use crate::build::setup_build;
mod build;

pub mod actions;
pub mod system;
pub mod ui;

fn main()
{
    // To Be Ignored, Just An Setup To Configure Some Assets
    setup_build();

    let window_config = WindowConfig {
        window_title: "AdvancedExample".to_string(),
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
    //bool is reffered to the rollback pages system, with "Mouse side buttons" or ("Alt" + "Arrows Keys") | (true = Page Rollback On), (false = Page Rollback Off)
    let mut input_handler = InputHandler::new(true);
    // TransitionType::Slide second arg, 0 = Down \ 1 = Up \ 2 = Right \ 3 = Left
    let mut app_state = AppState::new(PageId::Page1, window_modules.canvas.window().size());
    let mut page_data = PageData::new(&app_state);

    let renderer_config = RendererConfig { canvas: window_modules.canvas, texture_creator: &window_modules.texture_creator, ttf_context: &window_modules.ttf_context, font_path: &window_modules.font_path, decrease_color_when_selected: Some((25, 25, 25)), selection_color: Some((0, 0, 200, 125)) };
    let mut renderer = Renderer::new(renderer_config);

    populate_page_data(&mut page_data);

    // Wrap the button_action function in a mutable closure so it can capture
    // additional context if needed. Passing a closure here allows the
    // button handler API to accept additional arguments beyond the default.
    let mut button_action_closure = |app_state: &mut AppState<PageId, ButtonId>, button_id: &ButtonId, page_data: &mut PageData<PageId, ButtonId>| button_action(app_state, button_id, page_data);

    loop
    {
        //using (900 / your_refresh_rate) to a very crispy experience
        std::thread::sleep(Duration::from_millis(900 / get_monitor_refresh_rate()));
        input_handler.handle_input(&mut window_modules.event_pump, &mut window_modules.clipboard_system, &mut page_data, &mut app_state, &mut button_action_closure);
        app_state.update_window_size(renderer.canvas.window().size().0, renderer.canvas.window().size().1);
        page_data.create_current_page(&mut app_state);
        renderer.render(&mut page_data, &mut app_state, &input_handler);
    }
}
