use std::time::Duration;
use rust_page_system::{Renderer, system::{input_handler::InputHandler, page_system::PageData, state::AppState, window::{create_window, get_monitor_refresh_rate, WindowConfig}}};
use sdl3::sys::render::SDL_LOGICAL_PRESENTATION_STRETCH;
use crate::{actions::buttons_actions::button_action, ui::pages::PageId, system::setup_page_data::populate_page_data };



// To Be Ignored, Just An Setup To Configure Some Assets
use crate::build::setup_build;
mod build;



pub mod system;
pub mod actions;
pub mod ui;




fn main() 
{
    // To Be Ignored, Just An Setup To Configure Some Assets
    setup_build();

    let window_config = WindowConfig
    {
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
    // bool is reffered to the rollback pages system, with "Mouse side buttons" or ("Alt" + "Arrows Keys") | (false = Page Rollback On), (true = Page Rollback Off)
    let mut input_handler = InputHandler::new(false);
    let mut app_state = AppState::new(PageId::Page1);
    let mut page_data = PageData::new(&app_state);
    let mut renderer = Renderer::new(&mut window_modules.canvas, &window_modules.texture_creator, &window_modules.ttf_context, &window_modules.font_path, Some((25, 25, 25)), Some((0, 0, 200, 125)));

    populate_page_data(&mut page_data);

    loop 
    {
        //using (900 / your_refresh_rate) to a very crispy experience
        std::thread::sleep(Duration::from_millis(900 / get_monitor_refresh_rate()));
        input_handler.handle_input(&mut window_modules.event_pump, &mut window_modules.clipboard_system, &mut page_data, &mut app_state, button_action);
        app_state.update_window_size(renderer.canvas.window().size());
        page_data.create_current_page(&mut app_state);
        renderer.render(&page_data, &app_state, &input_handler);
    }
}
