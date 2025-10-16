use crate::{ButtonId, PageId};
use crate::
{
    system::{page_system::Page, renderer::render_page, window::WINDOW_DEFAULT_SCALE},
};
use sdl3::
{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    ttf::Sdl3TtfContext,
};










/// Global application state that holds UI and logic data.
pub struct AppState 
{
    pub current_page: PageId,
    pub vec_user_input: Vec<(String, PageId, ButtonId)>,
    pub capturing_input: (bool, Option<ButtonId>),
    pub window_size: (u32, u32),
}
/// Default Implementation Of AppState
impl Default for AppState { fn default() -> Self { Self::new() } }
impl AppState 
{
    /// Create The App State
    pub fn new() -> Self { Self { current_page: PageId::Page1, vec_user_input: Vec::new(), capturing_input: (false, None), window_size: (WINDOW_DEFAULT_SCALE.0, WINDOW_DEFAULT_SCALE.1) } }

    /// Populate vec_user_input
    pub fn push_vec_user_input(&mut self, user_input_needed: Vec<(PageId, ButtonId)>) { for pageid_and_user_input_needed in user_input_needed { self.vec_user_input.push((String::new(), pageid_and_user_input_needed.0, pageid_and_user_input_needed.1)) } }

    /// Returns the current window size
    pub fn current_window_size(&self) -> (u32, u32) { self.window_size }

    /// Called when user presses Enter or finishes typing
    pub fn submit_input(&mut self) { self.capturing_input.0 = false; self.capturing_input.1 = None; }

    /// Returns the button ID under the cursor (if any)
    pub fn page_button_at(&self, mouse_pos_x: f32, mouse_pos_y: f32) -> Option<ButtonId> 
    {
        let page = self.create_current_page();
        let page_button = page.button_at(mouse_pos_x, mouse_pos_y, self.window_size);
        if page_button.is_some() 
        {
            return page_button;
        }

        let persistent_page = Page::persistent_page();
        let persistent_page_button = persistent_page.button_at(mouse_pos_x, mouse_pos_y, self.window_size);
        if persistent_page_button.is_some()
        {
            return persistent_page_button
        }
            
        None
    }

    /// Append typed text into the current page's input slot(s).
    pub fn handle_text(&mut self, text: String) 
    {
        if !self.capturing_input.0 { return; }
        if let Some(capturing_input_button_id) = self.capturing_input.1 
        {
            for entry in self.vec_user_input.iter_mut() 
            {
                if entry.1 == self.current_page && entry.2 as usize == capturing_input_button_id as usize  { entry.0.push_str(&text); }
            }
       }
    }

    /// Handle a single backspace press for the current page's text input(s)
    pub fn handle_backspace(&mut self) 
    {
        if !self.capturing_input.0 { return; }
        if let Some(capturing_input_button_id) = self.capturing_input.1 
        {
            for entry in self.vec_user_input.iter_mut() 
            {
                if entry.1 == self.current_page && entry.2 as usize == capturing_input_button_id as usize && !entry.0.is_empty() { entry.0.pop(); }
            }
        }
    }

    /// Render All Pages
    pub fn render(&mut self, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, ttf_context: &Sdl3TtfContext) 
    {
        self.window_size = (canvas.window().size().0, canvas.window().size().1);
        let page = self.create_current_page();
        if page.has_persistant_page 
        {
            let mut need_vec_user_input = false;
            let mut vec_to_send = Vec::new();
            for user_input in &self.vec_user_input 
            { 
                if self.current_page == user_input.1 
                {
                    need_vec_user_input = true; 
                    vec_to_send.push(user_input.0.clone()); 
                } 
            }


        let mut vec_string_to_send = Vec::new();
        for user_input in &vec_to_send { vec_string_to_send.push(user_input.to_string()); }



            let persistent_page = if need_vec_user_input
            { Page::create_from_id(PageId::Persistent, Some(vec_string_to_send)) }
            else 
            { Page::create_from_id(PageId::Persistent, None) };

            render_page(&page, Some(&persistent_page), canvas, texture_creator, ttf_context);
        } 
        else
        {
            render_page(&page, None, canvas, texture_creator, ttf_context);
        }
    }

    /// Creates and Returns the current active page
    pub fn create_current_page(&self) -> Page 
    { 
        let mut need_vec_user_input = false;
        let mut vec_to_send = Vec::new();
        for user_input in &self.vec_user_input 
        { 
            if self.current_page == user_input.1 
            {
                need_vec_user_input = true; 
                vec_to_send.push(user_input.0.clone()); 
            } 
        }

        if need_vec_user_input
        { Page::create_from_id(self.current_page, Some(vec_to_send)) }
        else 
        { Page::create_from_id(self.current_page, None) }
    }

}
