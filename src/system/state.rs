use crate::system::
{
    page_system::{Button, Page},
    renderer::Renderer,
    window::WINDOW_DEFAULT_SCALE
};
use sdl3::
{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    ttf::Sdl3TtfContext,
};





/// Global application state that holds UI and logic data.
pub struct AppState<PageId, ButtonId>
{
    pub current_page: (PageId, bool),
    pub vec_user_input: Vec<(PageId, ButtonId)>,
    pub vec_user_input_string: Vec<String>,
    pub capturing_input: (bool, Option<ButtonId>),
    pub window_size: (u32, u32),
    pub persistent_elements: Vec<Page<PageId, ButtonId>>,
    pub all_pages: Vec<Page<PageId, ButtonId>>,
}
/// Default Implementation Of AppState
impl<PageId, ButtonId> AppState<PageId, ButtonId> where PageId: Copy + Eq, ButtonId: Copy + Eq,
{
    /// Create The App State
    pub fn new(page_id: PageId, has_user_input: bool) -> Self { Self { current_page: (page_id, has_user_input), vec_user_input: Vec::new(), vec_user_input_string: Vec::new(), capturing_input: (false, None), window_size: (WINDOW_DEFAULT_SCALE.0, WINDOW_DEFAULT_SCALE.1), persistent_elements: Vec::new(), all_pages: Vec::new()} }

    /// Returns the current window size
    pub fn current_window_size(&self) -> (u32, u32) { self.window_size }

    /// Called when user presses Enter or finishes typing
    pub fn submit_input(&mut self) { self.capturing_input.0 = false; self.capturing_input.1 = None; }

    /// Define Persistant Page
    pub fn define_persistent_elements(&mut self, persistent_elements: Vec<Page<PageId, ButtonId>>) { self.persistent_elements = persistent_elements }

    /// Populate all_buttons
    pub fn populate_and_update_all_pages(&mut self, all_pages: Vec<Page<PageId, ButtonId>>) { self.all_pages = all_pages }

    /// Populate vec_user_input
    pub fn push_vec_user_input(&mut self, user_input_needed: Vec<(PageId, ButtonId)>) 
    { 
        for pageid_and_user_input_needed in user_input_needed 
        { 
            self.vec_user_input.push((pageid_and_user_input_needed.0, pageid_and_user_input_needed.1));
            self.vec_user_input_string.push(String::new());
        } 
    }

    /// Returns the button ID under the cursor (if any)
    pub fn page_button_at(&self, mouse_pos_x: f32, mouse_pos_y: f32) -> Option<ButtonId> 
    {
        let mut buttons = Vec::new();
        for persistent_elements in &self.persistent_elements 
        {
            buttons.push(&persistent_elements.buttons);
        };
        for pages in &self.all_pages
        {
            buttons.push(&pages.buttons);
        };

        let mut buttons_to_be_evaluated = Vec::new();
        for page in &self.all_pages
        {
            if page.id == self.current_page.0
            {
                buttons_to_be_evaluated.push(&page.buttons);
            };

            if page.has_persistent_elements.0 && page.id == self.current_page.0 && let Some(vec_of_pageid) = &page.has_persistent_elements.1
            {
                for (index, pageid) in vec_of_pageid.iter().enumerate()
                {
                    if *pageid == self.persistent_elements[index].id
                    {
                        buttons_to_be_evaluated.push(&self.persistent_elements[index].buttons)
                    }
                }
            };
        }

    
        Button::button_at(buttons_to_be_evaluated, mouse_pos_x, mouse_pos_y, self.window_size)
    }

    /// Append typed text into the current page's input slot(s).
    pub fn handle_text(&mut self, text: String) 
    {
        if !self.capturing_input.0 { return; }
        if let Some(button_id) = self.capturing_input.1 
        {
            for (index, pageid_buttonid) in self.vec_user_input.iter_mut().enumerate()
            {
                if pageid_buttonid.0 == self.current_page.0 && pageid_buttonid.1 == button_id { self.vec_user_input_string[index].push_str(&text);}
            }
       }
    }

    /// Handle a single backspace press for the current page's text input(s)
    pub fn handle_backspace(&mut self) 
    {
        if !self.capturing_input.0 { return; }
        if let Some(capturing_input_button_id) = self.capturing_input.1 
        {
            for (index, pageid_buttonid) in self.vec_user_input.iter_mut().enumerate()
            {
                if pageid_buttonid.0 == self.current_page.0 && pageid_buttonid.1 == capturing_input_button_id && !self.vec_user_input_string[index].is_empty() { self.vec_user_input_string[index].pop(); }
            }
        }
    }

    /// Render All Pages
    pub fn render(&mut self, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, ttf_context: &Sdl3TtfContext) 
    {
        self.window_size = (canvas.window().size().0, canvas.window().size().1);
        for page in &self.all_pages
        {
            if self.current_page.0 == page.id && !page.has_persistent_elements.0
            {
                Renderer::render_page(page, None, canvas, texture_creator, ttf_context);
            }

            if self.current_page.0 == page.id && page.has_persistent_elements.0 && let Some(vec_of_pageid) = &page.has_persistent_elements.1
            {
                let mut vec_persistent_elements = Vec::new();
                for (index, pageid) in vec_of_pageid.iter().enumerate()
                {
                    if *pageid == self.persistent_elements[index].id
                    {
                        vec_persistent_elements.push(&self.persistent_elements[index]);
                    }
                }
                Renderer::render_page(page, Some(vec_persistent_elements), canvas, texture_creator, ttf_context);
            }
        }
    }
}
