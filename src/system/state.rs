use crate::
{
    actions::buttons_actions::button_action,
    system::renderer::render_page,
    ui::pages::{ButtonId, Page, PageId}
};
use sdl3::
{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    ttf::Sdl3TtfContext,
};





impl Page 
{
    pub fn button_at(&self, x: f32, y: f32, window_size: (u32, u32)) -> Option<ButtonId> 
    {
        if let Some(vec_buttons) = &self.buttons 
        {
            let x_scaled = x * (1920.00 / window_size.0 as f32);
            let y_scaled = y * (1080.00 / window_size.1 as f32);
            for button in vec_buttons 
            {
                if x_scaled >= button.rect.x as f32 && x_scaled <= (button.rect.x + button.rect.w) as f32 && y_scaled >= button.rect.y as f32 && y_scaled <= (button.rect.y + button.rect.h) as f32 
                {
                    return Some(button.id);
                }
            }
        }
        None
    }

    pub fn create_from_id(id: PageId, user_input: Vec<(String, PageId, ButtonId)>) -> Self 
    {
        match id 
        {
            PageId::Persistent => Self::persistent_page(),
            PageId::Page1 => 
            {
                let mut vec_to_send = Vec::new();
                for string_and_id in user_input 
                {
                    if string_and_id.1 == PageId::Page1 
                    {
                        vec_to_send.push(string_and_id.0);
                    };
                }
                if vec_to_send.is_empty() { vec_to_send.push(String::new()) };
                Self::page_1(vec_to_send)
            }
            PageId::Page2 => 
            {
                let mut vec_string_to_send = Vec::new();
                for string_and_id in user_input 
                {
                    if string_and_id.1 == PageId::Page2 { vec_string_to_send.push(string_and_id.0); };
                }
                if vec_string_to_send.is_empty() { vec_string_to_send.push(String::new()) };
                Self::page_2(vec_string_to_send)
            }
            PageId::Page2SubPage => Self::subpage_page2(),
        }
    }
}





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
    pub fn new() -> Self { Self { current_page: PageId::Page1, vec_user_input: Vec::new(), capturing_input: (false, None), window_size: (1920, 1080) } }

    /// Returns the button ID under the cursor (if any)
    pub fn page_button_at(&self, mouse_pos_x: f32, mouse_pos_y: f32) -> Option<ButtonId> 
    {
        let page = self.create_current_page();
        let page_query_result = page.button_at(mouse_pos_x, mouse_pos_y, self.window_size);
        if page_query_result.is_some() 
        {
            page_query_result
        } 
        else 
        {
            let persistent_page = Page::persistent_page();
            persistent_page.button_at(mouse_pos_x, mouse_pos_y, self.window_size)
        }
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
        for entry in self.vec_user_input.iter_mut() 
        {
            if entry.1 == self.current_page && !entry.0.is_empty() { entry.0.pop(); }
        }
    }

    /// Render All Pages
    pub fn render(&mut self, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, ttf_context: &Sdl3TtfContext) 
    {
        self.window_size = (canvas.window().size().0, canvas.window().size().1);
        let page = self.create_current_page();
        if page.has_persistant_page 
        {
            let persistent_page = Page::create_from_id(PageId::Persistent, self.vec_user_input.clone());
            render_page(&page, Some(&persistent_page), canvas, texture_creator, ttf_context);
        } 
        else
        {
            render_page(&page, None, canvas, texture_creator, ttf_context);
        }
    }

    /// Populate vec_user_input
    pub fn push_vec_user_input(&mut self, user_input_needed: Vec<(PageId, ButtonId)>) { for pageid_and_user_input_needed in user_input_needed { self.vec_user_input.push((String::new(), pageid_and_user_input_needed.0, pageid_and_user_input_needed.1)) } }

    /// Creates and Returns the current active page
    pub fn create_current_page(&self) -> Page { Page::create_from_id(self.current_page, self.vec_user_input.clone()) }

    /// Returns the current window size
    pub fn current_window_size(&self) -> (u32, u32) { self.window_size }

    /// Handles what happens when a button is clicked
    pub fn handle_action(&mut self, button_id: ButtonId) { button_action(self, button_id); }
    
    /// Called when user presses Enter or finishes typing
    pub fn submit_input(&mut self) { self.capturing_input.0 = false; }
}
