use std::{fmt::Debug,process::exit};
use crate::{system::page_system::PageData,AppState};
use sdl3::{clipboard::ClipboardUtil,event::Event,keyboard::{Keycode,Mod},mouse::MouseButton,EventPump};





#[derive(Debug,Clone)]
pub enum InputEvent
{
    Click(f32,f32),
    Text(String),
    Backspace,
    Submit,
    Front,
    Back,
    Paste,
    Copy,
    Cut,
    SelectAll,
    DeleteAll,
    CursorLeft(bool),
    CursorRight(bool),
    ExitCapturingInput,
    Quit,
    None,
}
pub struct InputHandler<PageId,ButtonId>
{
    pub cursor_pos:usize,
    pub selection:Option<(usize,usize)>,
    disable_rollback_pages:bool,
    _pageid:Option<PageId>,
    _buttonid:Option<ButtonId>,
}





impl<PageId: Copy+Eq+Debug, ButtonId: Copy+Eq+Debug> InputHandler<PageId,ButtonId>
{
    pub fn new(disable_rollback_pages: bool)->Self { Self {cursor_pos: 0, selection: None, disable_rollback_pages, _pageid:None, _buttonid:None} }

    pub fn poll(&self, event_pump: &mut EventPump)-> InputEvent
    {
        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::Quit{..}                                                         => return InputEvent::Quit,
                Event::TextInput{text,..}                                       => return InputEvent::Text(text),
                Event::MouseButtonDown{mouse_btn:MouseButton::Left,x,y,..}    => return InputEvent::Click(x,y),
                Event::MouseButtonDown{mouse_btn:MouseButton::X1,..}                    => {if !self.disable_rollback_pages{return InputEvent::Back}},
                Event::MouseButtonDown{mouse_btn:MouseButton::X2,..}                    => {if !self.disable_rollback_pages{return InputEvent::Front}},
                Event::KeyDown{keycode:Some(Keycode::Backspace),keymod,..}         => {if keymod.intersects(Mod::LCTRLMOD|Mod::RCTRLMOD){return InputEvent::DeleteAll}else{return InputEvent::Backspace}},
                Event::KeyDown{keycode:Some(Keycode::Return),..}                        => return InputEvent::Submit,
                Event::KeyDown{keycode:Some(Keycode::C),keymod,..}                 => {if keymod.intersects(Mod::LCTRLMOD|Mod::RCTRLMOD){return InputEvent::Copy}},
                Event::KeyDown{keycode:Some(Keycode::X),keymod,..}                 => {if keymod.intersects(Mod::LCTRLMOD|Mod::RCTRLMOD){return InputEvent::Cut}},
                Event::KeyDown{keycode:Some(Keycode::V),keymod,..}                 => {if keymod.intersects(Mod::LCTRLMOD|Mod::RCTRLMOD){return InputEvent::Paste}},
                Event::KeyDown{keycode:Some(Keycode::A),keymod,..}                 => {if keymod.intersects(Mod::LCTRLMOD|Mod::RCTRLMOD){return InputEvent::SelectAll}},
                Event::KeyDown{keycode:Some(Keycode::Left),keymod,..}              => return InputEvent::CursorLeft(keymod.intersects(Mod::LSHIFTMOD|Mod::RSHIFTMOD)),
                Event::KeyDown{keycode:Some(Keycode::Right),keymod,..}             => return InputEvent::CursorRight(keymod.intersects(Mod::LSHIFTMOD|Mod::RSHIFTMOD)),
                Event::KeyDown{keycode:Some(Keycode::Escape),..}                        => return InputEvent::ExitCapturingInput,
                _=>{}
            }
        }
        InputEvent::None
    }

    pub fn handle_input(&mut self, event_pump: &mut EventPump, clipboard: &mut ClipboardUtil, page_data: &mut PageData<PageId, ButtonId>, app_state: &mut AppState<PageId,ButtonId>, button_action: fn(&mut AppState<PageId, ButtonId>, &ButtonId, &mut PageData<PageId, ButtonId>))
    {
        match self.poll(event_pump)
        {
            InputEvent::Click(x,y)        => {if let Some(id)=page_data.page_button_at(app_state,x,y){button_action(app_state,&id,page_data);if app_state.capturing_input.0{self.cursor_pos=self.len_of_current_input(app_state,page_data,id);self.selection=None}}else{app_state.capturing_input=(false,None);self.selection=None}},
            InputEvent::Text(s)             => self.insert_text(&s,app_state,page_data),
            InputEvent::Backspace                   => self.backspace(app_state,page_data),
            InputEvent::Submit                      => app_state.capturing_input=(false,None),
            InputEvent::Front                       => self.history(true,app_state,page_data),
            InputEvent::Back                        => self.history(false,app_state,page_data),
            InputEvent::Paste                       => self.paste(Some(clipboard),app_state,page_data),
            InputEvent::Copy                        => self.copy(Some(clipboard),app_state,page_data,false),
            InputEvent::Cut                         => self.copy(Some(clipboard),app_state,page_data,true),
            InputEvent::SelectAll                   => self.select_all(app_state,page_data),
            InputEvent::DeleteAll                   => self.delete_all(app_state,page_data),
            InputEvent::CursorLeft(shift)     => self.move_cursor(false, shift, app_state, page_data),
            InputEvent::CursorRight(shift)    => self.move_cursor(true,  shift, app_state, page_data),
            InputEvent::ExitCapturingInput          => app_state.capturing_input=(false,None),
            InputEvent::Quit                        => exit(0),
            _=>{}
        }
    }

    pub fn insert_text(&mut self, text: &str, app: &AppState<PageId,ButtonId>, data: &mut PageData<PageId, ButtonId>)
    {
        if !app.capturing_input.0{return}
        let Some(id) = app.capturing_input.1 else {return};
        for(page_id, button_id, string)in &mut data.vec_user_input
        {
            if *page_id == app.current_page && *button_id == id
            {
                if let Some((s,e)) = self.selection.take()
                {
                    let s=s.min(string.len());
                    let e=e.min(string.len());
                    string.replace_range(s..e,text);
                    self.cursor_pos=s+text.len()
                }
                else
                {
                    let i=self.cursor_pos.min(string.len());
                    string.insert_str(i,text);
                    self.cursor_pos=i+text.len()
                }
                break;
            }
        }
        data.update_vec_user_input_string()
    }

    pub fn backspace(&mut self, app: &mut AppState<PageId, ButtonId>, data: &mut PageData<PageId, ButtonId>)
    {
        if !app.capturing_input.0 {return} 
        let Some(id)=app.capturing_input.1 else {return};
        for(page_id,button_id, string)in &mut data.vec_user_input
        {
            if *page_id == app.current_page && *button_id == id
            {
                if let Some((s,e)) = self.selection.take()
                {
                    let s=s.min(string.len());
                    let e=e.min(string.len());
                    string.replace_range(s..e,"");
                    self.cursor_pos=s
                }
                else if self.cursor_pos>0
                {
                    string.remove(self.cursor_pos-1);self.cursor_pos-=1
                }
                break;
            }
        }
        data.update_vec_user_input_string()
    }

    pub fn paste(&mut self, option_clipboard_util: Option<&mut ClipboardUtil>, app: &AppState<PageId,ButtonId>, data: &mut PageData<PageId,ButtonId>)
    {
        if let Some(clipboard_util) = option_clipboard_util && let Ok(txt) = clipboard_util.clipboard_text() { self.insert_text(&txt, app, data); }
    }

    pub fn copy(&mut self, option_clipboard_util: Option<&mut ClipboardUtil>, app: &AppState<PageId, ButtonId>, data: &mut PageData<PageId, ButtonId>, cut: bool)
    {
        if !app.capturing_input.0{return}let Some(id)=app.capturing_input.1 else{return};
        if let Some(clipboard_util)= option_clipboard_util
        {
            for(page_id,button_id, string) in &mut data.vec_user_input
            {
                if *page_id == app.current_page && *button_id == id
                {
                    let (character_selected,e)=self.selection.unwrap_or((0, string.len()));
                    let character_selected= character_selected.min(string.len());
                    let e=e.min(string.len());
                    let sub= string[character_selected..e].to_string();
                    clipboard_util.set_clipboard_text(&sub.clone()).unwrap_or(());
                    if cut
                    {
                        string.replace_range(character_selected..e,"");
                        self.cursor_pos= character_selected;
                        self.selection= None
                    }
                    break;
                }
            }
        }
        data.update_vec_user_input_string()
    }

    pub fn select_all(&mut self, app: &AppState<PageId, ButtonId>, data: &mut PageData<PageId, ButtonId>)
    {
        if !app.capturing_input.0{return};
        let Some(id) = app.capturing_input.1 else {return};
        for(page_id,button_id, string) in &mut data.vec_user_input
        {
            if *page_id == app.current_page && *button_id == id
            {
                self.selection = Some((0, string.len()));
                self.cursor_pos= string.len();
                break;
            }
        }
    }

    pub fn delete_all(&mut self, app: &AppState<PageId, ButtonId>, data: &mut PageData<PageId, ButtonId>)
    {
        if !app.capturing_input.0{return};
        let Some(id) = app.capturing_input.1 else {return};
        for(page_id, button_id, string) in &mut data.vec_user_input
        {
            if *page_id == app.current_page && *button_id == id
            {
                string.clear();
                self.cursor_pos=0;
                self.selection=None;
                break;
            }
        }
        data.update_vec_user_input_string()
    }

    pub fn move_cursor(&mut self, right: bool, shift_held: bool, app: &AppState<PageId, ButtonId>, data: &mut PageData<PageId, ButtonId>)
    {
        if !app.capturing_input.0{return};
        let Some(id)=app.capturing_input.1 else{return};
        let mut text_length = 0;
        for(page_id, button_id, string) in &mut data.vec_user_input
        {
            if *page_id == app.current_page && *button_id == id 
            {
                text_length = string.len();
                break;
            }
        }
    
        // SHIFT is held → selection mode
        if shift_held
        {
            if self.selection.is_none()
            {
                // Start a new selection from the current cursor position
                self.selection = Some((self.cursor_pos, self.cursor_pos));
            }
    
            if let Some((start, end)) = self.selection
            {
                // Compute new position based on direction
                let new_position = if right
                {
                    end.saturating_add(1).min(text_length)
                }
                else
                {
                    start.saturating_sub(1)
                };
    
                // Update selection range and cursor
                if right
                {
                    self.selection = Some((start, new_position));
                    self.cursor_pos = new_position;
                }
                else
                {
                    self.selection = Some((new_position, end));
                    self.cursor_pos = new_position;
                }
            }
        }
        else
        {
            // No SHIFT pressed → normal cursor movement
            self.selection = None;
            self.cursor_pos = if right
            {
                (self.cursor_pos + 1).min(text_length)
            }
            else
            {
                self.cursor_pos.saturating_sub(1)
            };
        }
    }

    pub fn len_of_current_input(&self, app: &AppState<PageId, ButtonId>, data: &PageData<PageId, ButtonId>, id: ButtonId) -> usize
    {
        for(page_id, button_id, string) in &data.vec_user_input
        {
            if *page_id == app.current_page && *button_id == id {return string.len()}
        }
        0
    }

    pub fn history(&self, front: bool, app: &mut AppState<PageId, ButtonId>, data: &mut PageData<PageId, ButtonId>)
    {
        if !data.page_history.0.is_empty() && !app.capturing_input.0
        {
            if front
            {
                if data.page_history.1 + 1 < data.page_history.0.len() { data.page_history.1+=1 };
                if let Some(page_id) = data.page_history.0.get(data.page_history.1) { app.current_page= *page_id }
            }
            else
            {
                if data.page_history.1 > 0 { data.page_history.1 -= 1 };
                if let Some(page_id) = data.page_history.0.get(data.page_history.1) { app.current_page= *page_id} 
            }
        }
    }
}

