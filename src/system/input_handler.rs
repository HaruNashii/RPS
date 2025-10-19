use crate::{system::page_system::PageData, AppState};
use sdl3::
{
    EventPump,
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton
};





#[derive(Debug, Clone)]
pub enum InputEvent 
{
    Click(f32, f32),
    Text(String),    
    Backspace,       
    Submit,          
    Quit,
    None,
}
pub struct InputHandler<PageId, ButtonId> { _pageid: Option<PageId>, _buttonid: Option<ButtonId>}
impl<PageId, ButtonId> Default for InputHandler<PageId, ButtonId> where PageId: Copy + Eq, ButtonId: Copy + Eq, { fn default() -> Self { Self::new() } }
impl<PageId, ButtonId> InputHandler<PageId, ButtonId> where PageId: Copy + Eq, ButtonId: Copy + Eq,
{
    pub fn new() -> Self { Self { _pageid: None, _buttonid: None}}

    pub fn poll(&self, event_pump: &mut EventPump) -> InputEvent 
    {
        for event in event_pump.poll_iter() 
        {
            match event 
            {
                Event::Quit { .. } => return InputEvent::Quit,
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => return InputEvent::Click(x, y),
                Event::TextInput { text, .. } => return InputEvent::Text(text),
                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => return InputEvent::Backspace,
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => return InputEvent::Submit,
                _ => {}
            }
        }
        InputEvent::None
    }

    /// Called when user presses Enter or finishes typing
    pub fn submit_input(&mut self, app_state: &mut AppState<PageId, ButtonId>) { app_state.capturing_input = (false, None); }

    /// Append typed text into the current page's input slot(s).
    pub fn handle_text(&mut self, text: String, app_state: &AppState<PageId, ButtonId>, page_data: &mut PageData<PageId, ButtonId>) 
    { 
        let capturing_input = app_state.capturing_input;
        if !capturing_input.0 { return; }
        if let Some(current_buttonid) = capturing_input.1 
        {
            let current_page = app_state.current_page;
            let vec_user_input = &mut page_data.vec_user_input;

            for (pageid, buttonid, user_input) in vec_user_input
            {
                if *pageid == current_page.0 && *buttonid == current_buttonid 
                { 
                    user_input.push_str(&text);
                }
            }
            page_data.update_vec_user_input_string();
        }
        else 
        {
            println!("capturing_input buttonid is not setted");
        }
    }

    /// Handle a single backspace press for the current page's text input(s)
    pub fn handle_backspace(&mut self, app_state: &mut AppState<PageId, ButtonId>, page_data: &mut PageData<PageId, ButtonId>) 
    {
        let capturing_input = app_state.capturing_input;
        if !capturing_input.0 { return; }
        if let Some(current_buttonid) = capturing_input.1 
        {
            let current_page = app_state.current_page;
            let vec_user_input = &mut page_data.vec_user_input;
            for (pageid, buttonid, user_input) in vec_user_input
            {
                if *pageid == current_page.0 && *buttonid == current_buttonid  && !user_input.is_empty()
                { 
                    user_input.pop();
                }
            }
            page_data.update_vec_user_input_string();
        }
    }
}
