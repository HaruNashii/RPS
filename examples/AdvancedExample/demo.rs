use std::time::Duration;
use rust_page_system::
{
    Renderer,
    system::
    {
        input_handler::{InputEvent, InputHandler},
        page_system::PageData,
        state::AppState,
        window::{create_window, get_monitor_refresh_rate, WindowConfig}
    }
};
use crate::
{   
    actions::buttons_actions::button_action, 
    ui::pages::{ButtonId, PageId},
    system::setup_page_data::{populate_page_data, update_page_data} 
};





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
        hint_sdl3_vsync: true
    };
    let (mut canvas, mut event_pump, texture_creator, ttf_context) = create_window(window_config);
    let mut input_handler = InputHandler::<PageId, ButtonId>::new();
    let mut app_state = AppState::<PageId, ButtonId>::new(PageId::Page1, true);
    let mut page_data = PageData::new();
    let mut renderer = Renderer::<PageId, ButtonId>::new(&mut canvas, &texture_creator, &ttf_context);

    populate_page_data(&mut page_data);

    let refresh_rate = get_monitor_refresh_rate();
    'running: loop 
    {
        app_state.update_window_size(renderer.canvas.window().size());
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
        update_page_data(&mut page_data);
        renderer.render(&app_state, &page_data);
    }
}
