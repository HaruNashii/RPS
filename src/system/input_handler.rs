use std::{fmt::Debug, process::exit};
use crate::{system::page_system::PageData, AppState};
use sdl3::{event::Event, keyboard::{Keycode, Mod}, mouse::MouseButton, EventPump};





#[derive(Debug, Clone)]
pub enum InputEvent 
{
    Click(f32, f32),
    Text(String),    
    Backspace,       
    Submit,          
    Back,
    Front,
    ExitCapturingInput,
    AltPressed,
    Quit,
    None,
}
pub struct InputHandler<PageId, ButtonId> { disable_rollback_pages: bool, _pageid: Option<PageId>, _buttonid: Option<ButtonId>}
impl<PageId: Copy + Eq + Debug, ButtonId: Copy + Eq + Debug> InputHandler<PageId, ButtonId>
{
    /// Create a new
    pub fn new(disable_rollback_pages: bool) -> Self { Self { disable_rollback_pages, _pageid: None, _buttonid: None}}

    pub fn poll(&self, event_pump: &mut EventPump) -> InputEvent 
    {
        for event in event_pump.poll_iter() 
        {
            match event 
            {
                Event::Quit { .. } => return InputEvent::Quit,
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => return InputEvent::Click(x, y),
                Event::MouseButtonDown { mouse_btn: MouseButton::X1, .. } => { if !self.disable_rollback_pages {return InputEvent::Back} },
                Event::MouseButtonDown { mouse_btn: MouseButton::X2, .. } => { if !self.disable_rollback_pages {return InputEvent::Front} },
                Event::TextInput { text, .. } => return InputEvent::Text(text),
                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => return InputEvent::Backspace,
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => return InputEvent::Submit,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return InputEvent::ExitCapturingInput,
                Event::KeyDown { keycode: Some(keycode), keymod, .. } => 
                {
                    let alt_held = keymod.intersects(Mod::LALTMOD | Mod::RALTMOD);
                    if alt_held 
                    {
                        match keycode 
                        {
                            Keycode::Left => { return InputEvent::Back; }
                            Keycode::Right => { return InputEvent::Front; }
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }
        InputEvent::None
    }

    /// Handle every event called
    pub fn handle_input(&mut self, event_pump: &mut EventPump, page_data: &mut PageData<PageId, ButtonId>, app_state: &mut AppState<PageId, ButtonId>, button_action: fn(&mut AppState<PageId, ButtonId>, &ButtonId, app_data: &mut PageData<PageId, ButtonId>))
    {
        match self.poll(event_pump) 
        {
            InputEvent::Click(x, y)   => if let Some(button_id) = page_data.page_button_at(app_state, x, y) { button_action(app_state, &button_id, page_data); },
            InputEvent::Text(string)    => self.handle_text(string, app_state, page_data),
            InputEvent::Backspace               => self.handle_backspace(app_state, page_data),
            InputEvent::Submit                  => self.submit_input(app_state),
            InputEvent::Front                   => self.manage_history(true, app_state, page_data),
            InputEvent::Back                    => self.manage_history(false, app_state, page_data),
            InputEvent::ExitCapturingInput      => app_state.capturing_input = (false, None),
            InputEvent::AltPressed              => {},
            InputEvent::Quit                    => exit(0),
            InputEvent::None                    => {}
        }
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
                if *pageid == current_page && *buttonid == current_buttonid 
                { 
                    user_input.push_str(&text);
                }
            }
            //page_data.update_vec_user_input_string();
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
                if *pageid == current_page && *buttonid == current_buttonid  && !user_input.is_empty()
                { 
                    user_input.pop();
                }
            }
            //page_data.update_vec_user_input_string();
        }
    }

    pub fn manage_history(&self, front: bool, app_state: &mut AppState<PageId, ButtonId>, page_data: &mut PageData<PageId, ButtonId>)
    {
        if !page_data.page_history.0.is_empty() && !app_state.capturing_input.0
        {
            while page_data.page_history.0.len() > 10 { page_data.page_history.0.pop_front(); };
            if front
            {    
                if page_data.page_history.1 + 1 < page_data.page_history.0.len() { page_data.page_history.1 += 1; }; 
                if let Some(page) = page_data.page_history.0.get(page_data.page_history.1) { app_state.current_page = *page; }
            }
            else 
            {
                if page_data.page_history.1 > 0 { page_data.page_history.1 -= 1; }
                if let Some(page) = page_data.page_history.0.get(page_data.page_history.1) { app_state.current_page = *page; }
            };
        }
    }
}
