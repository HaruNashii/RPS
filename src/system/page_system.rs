use std::{collections::VecDeque, fmt::Debug};
use crate::{system::window::WINDOW_DEFAULT_SCALE, AppState};
use sdl3::{pixels::Color, rect::Rect};





type Rects = Option<Vec<(Color, (Rect, i32))>>;
type Texts = Option<Vec<(f64, (i32, i32), String, Color)>>;
pub type Buttons<ButtonId> = Option<Vec<Button<ButtonId>>>;
type Images = Option<Vec<((i32, i32), (u32, u32), String)>>;
#[derive(PartialEq, Debug, Clone)]
pub struct Page<PageId, ButtonId> 
{
    pub has_persistent_elements: (bool, Option<Vec<PageId>>),
    pub id: PageId,
    pub background_color: Option<Color>,
    pub rects: Rects,
    pub buttons: Buttons<ButtonId>,
    pub texts: Texts,
    pub images: Images,
}





#[derive(PartialEq, Debug, Clone)]
pub struct PageData<PageId, ButtonId>
{
    pub vec_user_input: Vec<(PageId, ButtonId, String)>,
    pub vec_user_input_string: Vec<String>,
    pub persistent_elements: Vec<Page<PageId, ButtonId>>,
    pub all_pages: Vec<Page<PageId, ButtonId>>,
    pub page_history: (VecDeque<PageId>, usize),
}
impl<PageId: Copy + Eq, ButtonId: Copy + Eq + Debug> PageData<PageId, ButtonId>
{
    /// Define Persistant Page
    pub fn new(app_state: &AppState<PageId, ButtonId>) -> Self { Self {vec_user_input: Vec::new(), vec_user_input_string: Vec::new(), persistent_elements: Vec::new(), all_pages: Vec::new(), page_history: (VecDeque::from([app_state.current_page]),  0)} }

    /// Define Persistant Page
    pub fn define_persistent_elements(&mut self, persistent_elements: Vec<Page<PageId, ButtonId>>) { self.persistent_elements = persistent_elements }

    /// Populate all_buttons
    pub fn populate_and_update_all_pages(&mut self, all_pages: Vec<Page<PageId, ButtonId>>) { self.all_pages = all_pages; }

    /// Populate vec_user_input
    pub fn push_vec_user_input(&mut self, user_input_needed: Vec<(PageId, ButtonId)>) 
    { 
        for pageid_and_user_input_needed in user_input_needed 
        { 
            self.vec_user_input.push((pageid_and_user_input_needed.0, pageid_and_user_input_needed.1, String::new()));
        } 
        self.update_vec_user_input_string();
    }

    /// Update vec_user_input_string
    pub fn update_vec_user_input_string(&mut self)
    {
        let strings: Vec<String> = self.vec_user_input.iter().map(|(_, _, s)| s.to_string()).collect();
        self.vec_user_input_string = strings;
    }

    /// Returns the button ID under the cursor (if any)
    pub fn page_button_at(&self, app_state: &AppState<PageId, ButtonId>, mouse_pos_x: f32, mouse_pos_y: f32) -> Option<ButtonId> 
    {
        let current_page = app_state.current_page;
        let window_size = app_state.window_size;
        let mut buttons = Vec::new();
        for persistent_elements in &self.persistent_elements { buttons.push(&persistent_elements.buttons); };
        for pages in &self.all_pages { buttons.push(&pages.buttons); };

        let mut buttons_to_be_evaluated = Vec::new();
        for page in &self.all_pages
        {
            if page.id == current_page
            {
                buttons_to_be_evaluated.push(&page.buttons);
            };

            if page.has_persistent_elements.0 && page.id == current_page && let Some(vec_of_pageid) = &page.has_persistent_elements.1
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
        Button::button_at(buttons_to_be_evaluated, mouse_pos_x, mouse_pos_y, window_size)
    }

}






#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Button<ButtonId>
{
    pub enabled: bool,
    pub color: Color,
    pub rect: Rect,
    pub radius: i32,
    pub id: ButtonId,
}

impl<ButtonId: Copy + Eq + Debug> Button<ButtonId>
{
    pub fn button_at(option_vec_of_buttons: Vec<&Buttons<ButtonId>>, mouse_pos_x: f32, mouse_pos_y: f32, window_size: (u32, u32)) -> Option<ButtonId> 
    {
        for result_vec_of_buttons in option_vec_of_buttons.into_iter().flatten()
        {
            let x_scaled = mouse_pos_x * (WINDOW_DEFAULT_SCALE.0 as f32 / window_size.0 as f32);
            let y_scaled = mouse_pos_y * (WINDOW_DEFAULT_SCALE.1 as f32 / window_size.1 as f32);
            for button in result_vec_of_buttons 
            {
                if x_scaled >= button.rect.x as f32 && x_scaled <= (button.rect.x + button.rect.w) as f32 && y_scaled >= button.rect.y as f32 && y_scaled <= (button.rect.y + button.rect.h) as f32 
                {
                    return Some(button.id);
                }
            }
        }
        None
    }
}
