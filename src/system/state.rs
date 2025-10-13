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

    fn need_user_input() -> Option<Vec<PageId>> 
    {
        Some(vec![PageId::Page1, PageId::Page2])
    }

    fn from_id(id: PageId, user_input: Vec<(String, PageId)>) -> Self 
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
    pub persistent_page: Page,
    pub vec_user_input: Vec<(String, PageId)>,
    pub capturing_input: bool,
    pub window_size: (u32, u32),
}
/// Default Implementation Of AppState
impl Default for AppState { fn default() -> Self { Self::new() } }
impl AppState 
{
    /// Create The App State
    pub fn new() -> Self 
    {
        let mut default_self = Self { current_page: PageId::Page1, persistent_page: Page::persistent_page(), vec_user_input: Vec::new(), capturing_input: false, window_size: (1920, 1080) };
        // Populate vec_user_input
        let option_vec_of_pages = Page::need_user_input();
        if let Some(vec_of_pages_id) = option_vec_of_pages 
        {
            let mut number_of_strings_needed: Vec<(String, PageId)> = Vec::new();
            for page_id in vec_of_pages_id { number_of_strings_needed.push((String::new(), page_id)); }
            default_self.vec_user_input = number_of_strings_needed;
        };
        default_self
    }

    /// Returns the button ID under the cursor (if any)
    pub fn page_at(&self, x: f32, y: f32) -> Option<ButtonId> 
    {
        let page = self.create_current_page();
        let page_query_result = page.button_at(x, y, self.window_size);
        if page_query_result.is_some() 
        {
            page_query_result
        } 
        else 
        {
            let persistent_page = Page::persistent_page();
            persistent_page.button_at(x, y, self.window_size)
        }
    }

    /// Append typed text into the current page's input slot(s).
    pub fn handle_text(&mut self, text: String) 
    {
        if !self.capturing_input { return; }
        for entry in self.vec_user_input.iter_mut() 
        {
            if entry.1 == self.current_page { entry.0.push_str(&text); }
        }
    }

    /// Handle a single backspace press for the current page's text input(s)
    pub fn handle_backspace(&mut self) 
    {
        if !self.capturing_input { return; }
        for entry in self.vec_user_input.iter_mut() 
        {
            if entry.1 == self.current_page && !entry.0.is_empty() 
            {
                entry.0.pop();
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
            render_page(&page, Some(&self.persistent_page), canvas, texture_creator, ttf_context);
        } 
        else
        {
            render_page(&page, None, canvas, texture_creator, ttf_context);
        }
    }

    /// Creates and Returns the current active page
    pub fn create_current_page(&self) -> Page { Page::from_id(self.current_page, self.vec_user_input.clone()) }

    /// Returns the current window size
    pub fn current_window_size(&self) -> (u32, u32) { self.window_size }

    /// Handles what happens when a button is clicked
    pub fn handle_action(&mut self, button_id: ButtonId) { button_action(self, button_id); }
    
    /// Called when user presses Enter or finishes typing
    pub fn submit_input(&mut self) { self.capturing_input = false; }
}
