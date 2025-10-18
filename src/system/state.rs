use crate::system::{
        page_system::{button_at, Page},
        renderer::render_page,
        window::WINDOW_DEFAULT_SCALE
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
    pub current_page: (usize, bool),
    pub vec_user_input: Vec<(String, usize, usize)>,
    pub vec_user_input_string: Vec<String>,
    pub capturing_input: (bool, Option<usize>),
    pub window_size: (u32, u32),
    pub all_pages: Vec<Page>,
}
/// Default Implementation Of AppState
impl Default for AppState { fn default() -> Self { Self::new() } }
impl AppState 
{
    /// Create The App State
    pub fn new() -> Self { Self { current_page: (1, false), vec_user_input: Vec::new(), vec_user_input_string: Vec::new(), capturing_input: (false, None), window_size: (WINDOW_DEFAULT_SCALE.0, WINDOW_DEFAULT_SCALE.1), all_pages: Vec::new()} }

    /// Returns the current window size
    pub fn current_window_size(&self) -> (u32, u32) { self.window_size }

    /// Called when user presses Enter or finishes typing
    pub fn submit_input(&mut self) { self.capturing_input.0 = false; self.capturing_input.1 = None; }

    /// Populate all_buttons
    pub fn populate_and_update_all_pages(&mut self, all_pages: Vec<Page>) { self.all_pages = all_pages }

    /// Populate vec_user_input
    pub fn push_vec_user_input(&mut self, user_input_needed: Vec<(usize, usize)>) 
    { 
        for pageid_and_user_input_needed in user_input_needed 
        { 
            self.vec_user_input.push((String::new(), pageid_and_user_input_needed.0, pageid_and_user_input_needed.1));
            self.vec_user_input_string.push(String::new());
        } 
    }

    /// Returns the button ID under the cursor (if any)
    pub fn page_button_at(&self, mouse_pos_x: f32, mouse_pos_y: f32) -> Option<usize> 
    {
        let mut page_buttons = Vec::new();
        for pages in &self.all_pages
        {
            page_buttons.push(pages.buttons.clone());
        }

        let mut buttons_to_be_evaluated = Vec::new();
        for page in &self.all_pages
        {
            if (self.current_page.1 && page.id == 0) || (page.id == self.current_page.0)
            {
                buttons_to_be_evaluated.push(page.buttons.clone());
            }
        }
    
        button_at(buttons_to_be_evaluated, mouse_pos_x, mouse_pos_y, self.window_size)
    }

    /// Append typed text into the current page's input slot(s).
    pub fn handle_text(&mut self, text: String) 
    {
        if !self.capturing_input.0 { return; }
        if let Some(capturing_input_button_id) = self.capturing_input.1 
        {
            for (index, entry) in self.vec_user_input.iter_mut().enumerate()
            {
                if entry.1 == self.current_page.0 && entry.2 == capturing_input_button_id { entry.0.push_str(&text);}
                if entry.1 == self.current_page.0 && entry.2 == capturing_input_button_id { self.vec_user_input_string[index].push_str(&text);}
            }
       }
    }

    /// Handle a single backspace press for the current page's text input(s)
    pub fn handle_backspace(&mut self) 
    {
        if !self.capturing_input.0 { return; }
        if let Some(capturing_input_button_id) = self.capturing_input.1 
        {
            for (index, entry) in self.vec_user_input.iter_mut().enumerate()
            {
                if entry.1 == self.current_page.0 && entry.2 == capturing_input_button_id && !entry.0.is_empty() { entry.0.pop(); }
                if entry.1 == self.current_page.0 && entry.2 == capturing_input_button_id && !self.vec_user_input_string[index].is_empty() { self.vec_user_input_string[index].pop(); }
            }
        }
    }

    /// Render All Pages
    pub fn render(&mut self, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, ttf_context: &Sdl3TtfContext) 
    {
        self.window_size = (canvas.window().size().0, canvas.window().size().1);
        
        for page in &self.all_pages
        {
            if self.current_page.0 == page.id && !page.has_persistant_page
            {
                render_page(page, None, canvas, texture_creator, ttf_context);
            }

            if self.current_page.0 == page.id && page.has_persistant_page
            {
                render_page(page, Some(&self.all_pages[0]), canvas, texture_creator, ttf_context);
            }
        }
    }
}
