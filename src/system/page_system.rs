use crate::{
    AppState,
    system::{scene_transition::TransitionType, window::WINDOW_DEFAULT_SCALE}
};
use sdl3::{pixels::Color, rect::Rect};
use std::{collections::VecDeque, fmt::Debug};

type PersistentElementsType<PageId, ButtonId> = Option<Vec<(PageId, fn() -> PersistentElements<PageId, ButtonId>)>>;
type Rects = Option<Vec<(Color, (Rect, i32))>>;
type Texts = Option<Vec<(f64, (i32, i32), String, Color)>>;
pub type Buttons<ButtonId> = Option<Vec<Button<ButtonId>>>;
type Images = Option<Vec<((i32, i32), (u32, u32), String)>>;
#[derive(PartialEq, Debug, Clone)]
pub struct Page<PageId, ButtonId>
{
    pub has_persistent_elements: PersistentElementsType<PageId, ButtonId>,
    pub has_userinput: Option<Vec<(PageId, ButtonId)>>,
    pub id: PageId,
    pub background_color: Option<Color>,
    pub rects: Rects,
    pub buttons: Buttons<ButtonId>,
    pub texts: Texts,
    pub images: Images
}
#[derive(PartialEq, Debug, Clone)]
pub struct PersistentElements<PageId, ButtonId>
{
    pub id: PageId,
    pub background_color: Option<Color>,
    pub rects: Rects,
    pub buttons: Buttons<ButtonId>,
    pub texts: Texts,
    pub images: Images
}

type OptionPageInputLinked<PageId, ButtonId> = Option<Vec<(PageId, fn(&mut Vec<String>) -> Page<PageId, ButtonId>)>>;
type OptionPageLinked<PageId, ButtonId> = Option<Vec<(PageId, fn() -> Page<PageId, ButtonId>)>>;
type PageInputLinked<PageId, ButtonId> = Vec<(PageId, fn(&mut Vec<String>) -> Page<PageId, ButtonId>)>;
type PageLinked<PageId, ButtonId> = Vec<(PageId, fn() -> Page<PageId, ButtonId>)>;
#[derive(PartialEq, Debug, Clone)]
pub struct PageData<PageId, ButtonId>
{
    pub vec_user_input: Vec<(PageId, ButtonId, String)>,
    pub vec_user_input_string: Vec<String>,
    pub page_history: (VecDeque<PageId>, usize),
    pub page_linked: PageLinked<PageId, ButtonId>,
    pub page_w_input_linked: PageInputLinked<PageId, ButtonId>,
    pub page_to_render: Option<Page<PageId, ButtonId>>,
    pub persistent_elements_to_render: Option<Vec<PersistentElements<PageId, ButtonId>>>
}
impl<PageId: Copy + Eq + Debug, ButtonId: Copy + Eq + Debug> PageData<PageId, ButtonId>
{
    /// Define PageData Default  Config
    pub fn new(app_state: &AppState<PageId, ButtonId>) -> Self
    {
        Self { vec_user_input: Vec::new(), vec_user_input_string: Vec::new(), persistent_elements_to_render: None, page_history: (VecDeque::from([app_state.current_page]), 0), page_linked: Vec::new(), page_w_input_linked: Vec::new(), page_to_render: None }
    }

    ///Link The Page With Your Determined PageId
    pub fn push_page_link(&mut self, option_page_linked_received: OptionPageLinked<PageId, ButtonId>, option_page_w_input_linked_received: OptionPageInputLinked<PageId, ButtonId>)
    {
        if let Some(page_linked_received) = option_page_linked_received
        {
            self.page_linked = page_linked_received;
        };
        if let Some(page_w_input_linked_received) = option_page_w_input_linked_received
        {
            self.page_w_input_linked = page_w_input_linked_received;
        };
    }

    //Create The Page Setted In THe AppState current_page
    pub fn create_current_page(&mut self, app_state: &mut AppState<PageId, ButtonId>)
    {
        while self.page_history.0.len() > 10
        {
            self.page_history.0.pop_front();
        }
        let mut page_to_render = self.create_page_from_id(app_state.current_page); // function call returns a new page
        if let Some(page_to_render) = &mut page_to_render
        {
            self.push_vec_user_input_per_page(page_to_render);
            if let Some(result) = &page_to_render.has_persistent_elements
                && self.persistent_elements_to_render.is_some()
            {
                self.persistent_elements_to_render = Some(result.iter().map(|(_, f)| f()).collect());
            }
        }
        self.page_to_render = page_to_render;
    }

    /// Create An Page With Your Passed Id, The Page Parsed Needs To Be Already Linked
    pub fn create_page_from_id(&mut self, page_to_create: PageId) -> Option<Page<PageId, ButtonId>>
    {
        let mut created_page = None;
        for (id, create_fn) in &self.page_linked
        {
            if *id == page_to_create
            {
                let page = create_fn();
                created_page = Some(page);
                if let Some(persistent_list) = &created_page.as_ref().unwrap().has_persistent_elements
                {
                    self.persistent_elements_to_render = Some(persistent_list.iter().map(|(_, make_persistent)| make_persistent()).collect());
                }
                break;
            }
        }

        if created_page.is_none()
        {
            for (id, create_fn_with_input) in &self.page_w_input_linked
            {
                if *id == page_to_create
                {
                    let page = create_fn_with_input(&mut self.vec_user_input_string);
                    created_page = Some(page);
                    if let Some(persistent_list) = &created_page.as_ref().unwrap().has_persistent_elements
                    {
                        self.persistent_elements_to_render = Some(persistent_list.iter().map(|(_, make_persistent)| make_persistent()).collect());
                    }
                    break;
                }
            }
        }
        created_page
    }

    /// Populate vec_user_input per page
    pub fn push_vec_user_input_per_page(&mut self, page: &mut Page<PageId, ButtonId>)
    {
        if let Some(has_userinput) = &page.has_userinput
        {
            for (pageid, buttonid) in has_userinput
            {
                let exists = self.vec_user_input.iter().any(|(pid, bid, _)| pid == pageid && bid == buttonid);
                if !exists
                {
                    self.vec_user_input.push((*pageid, *buttonid, String::new()));
                }
            }
            self.update_vec_user_input_string();
        }
    }

    /// Populate vec_user_input per vector of pages
    pub fn push_vec_user_input_per_vec(&mut self)
    {
        let mut changed = false;
        for tuple_page in &self.page_w_input_linked
        {
            let page = (tuple_page.1)(&mut self.vec_user_input_string);
            if let Some(has_userinput) = &page.has_userinput
            {
                for (pageid, buttonid) in has_userinput
                {
                    let exists = self.vec_user_input.iter().any(|(pid, bid, _)| pid == pageid && bid == buttonid);
                    if !exists
                    {
                        self.vec_user_input.push((*pageid, *buttonid, String::new()));
                        changed = true;
                    }
                }
            }
        }
        if changed
        {
            self.update_vec_user_input_string();
        }
    }

    /// Update vec_user_input_string
    pub fn update_vec_user_input_string(&mut self)
    {
        self.vec_user_input_string = self.vec_user_input.iter().map(|(_, _, s)| s.to_string()).collect();
    }

    /// Returns the button ID under the cursor (if any)
    pub fn page_button_at(&self, app_state: &AppState<PageId, ButtonId>, mouse_pos_x: f32, mouse_pos_y: f32) -> Option<ButtonId>
    {
        let window_size = app_state.window_size;
        let mut page_buttons = &None;
        if let Some(page_to_render) = &self.page_to_render
        {
            page_buttons = &page_to_render.buttons;
        }

        let mut buttons_to_be_evaluated = Vec::new();
        buttons_to_be_evaluated.push(page_buttons);

        if self.persistent_elements_to_render.is_some()
            && let Some(p) = &self.persistent_elements_to_render
        {
            for persistent_element in p
            {
                buttons_to_be_evaluated.push(&persistent_element.buttons)
            }
        };
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
    pub has_transition: Option<TransitionType>
}

impl<ButtonId: Copy + Eq + Debug> Button<ButtonId>
{
    /// See If The Mouse Position Has Some Button In The Same Place, If Not Return None
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
