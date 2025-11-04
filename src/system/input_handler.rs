use crate::{AppState, system::page_system::PageData};
use sdl3::{
    EventPump,
    clipboard::ClipboardUtil,
    event::Event,
    keyboard::{Keycode, Mod},
    mouse::MouseButton
};
use std::{fmt::Debug, process::exit};

#[derive(Debug, Clone)]
pub enum InputEvent
{
    Click,
    Text(String),
    Backspace,
    Submit,
    Front,
    Back,
    Paste,
    Copy,
    Cut,
    Undo,
    SelectAll,
    DeleteAll,
    CursorLeft(bool),
    CursorRight(bool),
    ExitCapturingInput,
    Quit,
    None
}

pub struct InputHandler<PageId, ButtonId>
{
    pub button_selected: Option<ButtonId>,
    pub mouse_position: (f32, f32),
    pub cursor_position: usize,
    pub text_selection_range: Option<(usize, usize)>,
    enable_rollback_pages: bool,
    input_history_stack: Vec<Vec<(PageId, ButtonId, String)>>
}

impl<PageId: Copy + Eq + Debug, ButtonId: Copy + Eq + Debug> InputHandler<PageId, ButtonId>
{
    pub fn new(enable_rollback_pages: bool) -> Self
    {
        Self { cursor_position: 0, text_selection_range: None, enable_rollback_pages, mouse_position: (0., 0.), button_selected: None, input_history_stack: Vec::new() }
    }

    pub fn poll(&self, event_pump: &mut EventPump) -> InputEvent
    {
        for event in event_pump.poll_iter()
        {
            match event
            {
                //mouse events
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => return InputEvent::Click,
                Event::MouseButtonDown { mouse_btn: MouseButton::X1, .. } =>
                {
                    if self.enable_rollback_pages
                    {
                        return InputEvent::Back;
                    }
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::X2, .. } =>
                {
                    if self.enable_rollback_pages
                    {
                        return InputEvent::Front;
                    }
                }

                //keyboard events
                Event::TextInput { text, .. } => return InputEvent::Text(text),
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => return InputEvent::Submit,
                Event::KeyDown { keycode: Some(Keycode::Backspace), keymod, .. } =>
                {
                    if keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD)
                    {
                        return InputEvent::DeleteAll;
                    }
                    else
                    {
                        return InputEvent::Backspace;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Z), keymod, .. } =>
                {
                    if keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD)
                    {
                        return InputEvent::Undo;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::C), keymod, .. } =>
                {
                    if keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD)
                    {
                        return InputEvent::Copy;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::X), keymod, .. } =>
                {
                    if keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD)
                    {
                        return InputEvent::Cut;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::V), keymod, .. } =>
                {
                    if keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD)
                    {
                        return InputEvent::Paste;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::A), keymod, .. } =>
                {
                    if keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD)
                    {
                        return InputEvent::SelectAll;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Left), keymod, .. } => return InputEvent::CursorLeft(keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD)),
                Event::KeyDown { keycode: Some(Keycode::Right), keymod, .. } => return InputEvent::CursorRight(keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD)),
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return InputEvent::ExitCapturingInput,

                //window events
                Event::Quit { .. } => return InputEvent::Quit,
                _ =>
                {}
            }
        }
        InputEvent::None
    }

    #[allow(clippy::type_complexity)]
    pub fn handle_input(&mut self, event_pump: &mut EventPump, clipboard_util: &mut ClipboardUtil, page_data: &mut PageData<PageId, ButtonId>, app_state: &mut AppState<PageId, ButtonId>, button_action: &mut dyn Fn(&mut AppState<PageId, ButtonId>, &ButtonId, &mut PageData<PageId, ButtonId>))
    {
        self.button_selected = page_data.page_button_at(app_state, event_pump.mouse_state().x(), event_pump.mouse_state().y());
        if let Some(active_button_id) = app_state.capturing_input.1
        {
            self.button_selected = Some(active_button_id)
        }

        match self.poll(event_pump)
        {
            InputEvent::Click =>
            {
                if let Some(button_id) = self.button_selected
                {
                    (*button_action)(app_state, &button_id, page_data);
                    if app_state.capturing_input.0
                    {
                        self.cursor_position = self.get_current_input_length(app_state, page_data, button_id);
                        self.text_selection_range = None
                    }
                }
                else
                {
                    app_state.capturing_input = (false, None);
                    self.text_selection_range = None
                }
            }
            InputEvent::Text(text_input) =>
            {
                self.push_state(page_data);
                self.insert_text(&text_input, app_state, page_data, false)
            }
            InputEvent::Backspace =>
            {
                self.push_state(page_data);
                self.backspace(app_state, page_data)
            }
            InputEvent::Paste =>
            {
                self.push_state(page_data);
                self.paste(Some(clipboard_util), app_state, page_data)
            }
            InputEvent::Cut =>
            {
                self.push_state(page_data);
                self.copy(Some(clipboard_util), app_state, page_data, true)
            }
            InputEvent::DeleteAll =>
            {
                self.push_state(page_data);
                self.delete_all(app_state, page_data)
            }
            InputEvent::Copy => self.copy(Some(clipboard_util), app_state, page_data, false),
            InputEvent::SelectAll => self.select_all(app_state, page_data),
            InputEvent::Undo => self.undo(page_data),
            InputEvent::CursorLeft(shift_held) => self.move_cursor(false, shift_held, app_state, page_data),
            InputEvent::CursorRight(shift_held) => self.move_cursor(true, shift_held, app_state, page_data),
            InputEvent::Submit => app_state.capturing_input = (false, None),
            InputEvent::Front => self.navigate_history(true, app_state, page_data),
            InputEvent::Back => self.navigate_history(false, app_state, page_data),
            InputEvent::ExitCapturingInput => app_state.capturing_input = (false, None),
            InputEvent::Quit => exit(0),
            _ =>
            {}
        }
    }

    // To Be Tweaked more, is_paste is giving weird outcome
    pub fn insert_text(&mut self, text_to_insert: &str, app_state: &AppState<PageId, ButtonId>, page_data: &mut PageData<PageId, ButtonId>, _is_paste: bool)
    {
        if !app_state.capturing_input.0
        {
            return;
        }
        let Some(active_button_id) = app_state.capturing_input.1
        else
        {
            return;
        };
        for (page_id, button_id, input_string) in &mut page_data.vec_user_input
        {
            if *page_id == app_state.current_page && *button_id == active_button_id
            {
                if let Some((anchor_index, cursor_index)) = self.text_selection_range.take()
                {
                    let (mut start_index, mut end_index) = if anchor_index <= cursor_index { (anchor_index, cursor_index) } else { (cursor_index, anchor_index) };
                    start_index = start_index.min(input_string.len());
                    end_index = end_index.min(input_string.len());
                    if start_index != end_index
                    {
                        input_string.replace_range(start_index..end_index, text_to_insert);
                        self.cursor_position = start_index + text_to_insert.len()
                    }
                    else
                    {
                        let insert_index = self.cursor_position.min(input_string.len());
                        input_string.insert_str(insert_index, text_to_insert);
                        self.cursor_position = insert_index + text_to_insert.len()
                    }
                }
                else
                {
                    let insert_index = self.cursor_position.min(input_string.len());
                    input_string.insert_str(insert_index, text_to_insert);
                    self.cursor_position = insert_index + text_to_insert.len()
                }
                break;
            }
        }
        page_data.update_vec_user_input_string()
    }

    pub fn backspace(&mut self, app_state: &mut AppState<PageId, ButtonId>, page_data: &mut PageData<PageId, ButtonId>)
    {
        if !app_state.capturing_input.0
        {
            return;
        }
        let Some(active_button_id) = app_state.capturing_input.1
        else
        {
            return;
        };
        for (page_id, button_id, input_string) in &mut page_data.vec_user_input
        {
            if *page_id == app_state.current_page && *button_id == active_button_id
            {
                if let Some((anchor_index, cursor_index)) = self.text_selection_range.take()
                {
                    let (mut start_index, mut end_index) = if anchor_index <= cursor_index { (anchor_index, cursor_index) } else { (cursor_index, anchor_index) };
                    start_index = start_index.min(input_string.len());
                    end_index = end_index.min(input_string.len());
                    if start_index != end_index
                    {
                        input_string.replace_range(start_index..end_index, "");
                        self.cursor_position = start_index
                    }
                }
                else if self.cursor_position > 0
                {
                    let remove_index = self.cursor_position - 1;
                    if remove_index < input_string.len()
                    {
                        input_string.remove(remove_index);
                        self.cursor_position -= 1
                    }
                }
                break;
            }
        }
        page_data.update_vec_user_input_string()
    }

    pub fn copy(&mut self, clipboard_util_option: Option<&mut ClipboardUtil>, app_state: &AppState<PageId, ButtonId>, page_data: &mut PageData<PageId, ButtonId>, is_cut_operation: bool)
    {
        if !app_state.capturing_input.0
        {
            return;
        }
        let Some(active_button_id) = app_state.capturing_input.1
        else
        {
            return;
        };
        if let Some(clipboard_util) = clipboard_util_option
        {
            for (page_id, button_id, input_string) in &mut page_data.vec_user_input
            {
                if *page_id == app_state.current_page
                    && *button_id == active_button_id
                    && !input_string.is_empty()
                    && let Some((anchor_index, cursor_index)) = self.text_selection_range
                {
                    let (mut start_index, mut end_index) = if anchor_index <= cursor_index { (anchor_index, cursor_index) } else { (cursor_index, anchor_index) };
                    start_index = start_index.min(input_string.len());
                    end_index = end_index.min(input_string.len());
                    if start_index < end_index
                    {
                        let selected_text = input_string[start_index..end_index].to_string();
                        let _ = clipboard_util.set_clipboard_text(&selected_text);
                        if is_cut_operation
                        {
                            input_string.replace_range(start_index..end_index, "");
                            self.cursor_position = start_index;
                            self.text_selection_range = None
                        }
                    }
                    break;
                }
            }
        }
        page_data.update_vec_user_input_string()
    }

    pub fn select_all(&mut self, app_state: &AppState<PageId, ButtonId>, page_data: &mut PageData<PageId, ButtonId>)
    {
        if !app_state.capturing_input.0
        {
            return;
        }
        let Some(active_button_id) = app_state.capturing_input.1
        else
        {
            return;
        };
        for (page_id, button_id, input_string) in &mut page_data.vec_user_input
        {
            if *page_id == app_state.current_page && *button_id == active_button_id
            {
                self.text_selection_range = Some((0, input_string.len()));
                self.cursor_position = input_string.len();
                break;
            }
        }
    }

    pub fn delete_all(&mut self, app_state: &AppState<PageId, ButtonId>, page_data: &mut PageData<PageId, ButtonId>)
    {
        if !app_state.capturing_input.0
        {
            return;
        }
        let Some(active_button_id) = app_state.capturing_input.1
        else
        {
            return;
        };
        for (page_id, button_id, input_string) in &mut page_data.vec_user_input
        {
            if *page_id == app_state.current_page && *button_id == active_button_id
            {
                input_string.clear();
                self.cursor_position = 0;
                self.text_selection_range = None;
                break;
            }
        }
        page_data.update_vec_user_input_string()
    }

    pub fn move_cursor(&mut self, move_right: bool, shift_held: bool, app_state: &AppState<PageId, ButtonId>, page_data: &mut PageData<PageId, ButtonId>)
    {
        if !app_state.capturing_input.0
        {
            return;
        }
        let Some(active_button_id) = app_state.capturing_input.1
        else
        {
            return;
        };
        let mut text_length = 0;
        for (page_id, button_id, input_string) in &mut page_data.vec_user_input
        {
            if *page_id == app_state.current_page && *button_id == active_button_id
            {
                text_length = input_string.len();
                break;
            }
        }
        if shift_held
        {
            if self.text_selection_range.is_none()
            {
                self.text_selection_range = Some((self.cursor_position, self.cursor_position))
            }
            if let Some((anchor_index, _cursor_index)) = self.text_selection_range
            {
                let new_cursor_index = if move_right { (self.cursor_position + 1).min(text_length) } else { self.cursor_position.saturating_sub(1) };
                self.text_selection_range = Some((anchor_index, new_cursor_index));
                self.cursor_position = new_cursor_index;
            }
        }
        else
        {
            self.text_selection_range = None;
            self.cursor_position = if move_right { (self.cursor_position + 1).min(text_length) } else { self.cursor_position.saturating_sub(1) };
        }
    }

    pub fn navigate_history(&self, move_forward: bool, app_state: &mut AppState<PageId, ButtonId>, page_data: &mut PageData<PageId, ButtonId>)
    {
        if !page_data.page_history.0.is_empty() && !app_state.capturing_input.0
        {
            if move_forward
            {
                if page_data.page_history.1 + 1 < page_data.page_history.0.len()
                {
                    page_data.page_history.1 += 1
                };
                if let Some(page_id) = page_data.page_history.0.get(page_data.page_history.1)
                {
                    app_state.current_page = *page_id
                }
            }
            else
            {
                if page_data.page_history.1 > 0
                {
                    page_data.page_history.1 -= 1
                };
                if let Some(page_id) = page_data.page_history.0.get(page_data.page_history.1)
                {
                    app_state.current_page = *page_id
                }
            }
        }
    }

    pub fn paste(&mut self, clipboard_util_option: Option<&mut ClipboardUtil>, app_state: &AppState<PageId, ButtonId>, page_data: &mut PageData<PageId, ButtonId>)
    {
        if let Some(clipboard_util) = clipboard_util_option
            && let Ok(clipboard_text) = clipboard_util.clipboard_text()
        {
            self.insert_text(&clipboard_text, app_state, page_data, true);
        }
    }

    pub fn push_state(&mut self, page_data: &PageData<PageId, ButtonId>)
    {
        self.input_history_stack.push(page_data.vec_user_input.clone())
    }

    pub fn undo(&mut self, page_data: &mut PageData<PageId, ButtonId>)
    {
        if let Some(previous_input_state) = self.input_history_stack.pop()
        {
            page_data.vec_user_input = previous_input_state;
            page_data.update_vec_user_input_string()
        }
    }

    pub fn get_current_input_length(&self, app_state: &AppState<PageId, ButtonId>, page_data: &PageData<PageId, ButtonId>, button_id: ButtonId) -> usize
    {
        for (page_id, button_id_in_vec, input_string) in &page_data.vec_user_input
        {
            if *page_id == app_state.current_page && *button_id_in_vec == button_id
            {
                return input_string.len();
            }
        }
        0
    }
}
