use crate::{
    AppState,
    system::{scene_transition::TransitionType, window::WINDOW_DEFAULT_SCALE}
};
use sdl3::{pixels::Color, rect::Rect};
use std::rc::Rc;
use std::{collections::VecDeque, fmt::Debug};

/// Type for optional list of persistent elements factories.
/// Each entry associates a `PageId` with a boxed closure that constructs
/// the corresponding `PersistentElements` when invoked. Using boxed
/// closures instead of plain function pointers allows callers to
/// capture additional data via the closure, enabling more flexible
/// APIs.
// Optionally linked persistent elements.  Removed trailing comma in the tuple type.
// Optionally linked persistent elements. Removed trailing comma and added space between closing
// angle brackets to avoid parsing `>>` as a right-shift operator.
// Optionally linked persistent elements.
// Each entry associates a `PageId` with a boxed closure that constructs
// the corresponding `PersistentElements` when invoked. Using boxed closures
// instead of plain function pointers allows callers to capture additional
// data via the closure, enabling more flexible APIs.
type PersistentElementsType<PageId, ButtonId> = Option<Vec<(PageId, Rc<dyn Fn() -> PersistentElements<PageId, ButtonId>>)>>;
type Rects = Option<Vec<(Color, (Rect, i32))>>;
type Texts = Option<Vec<(f64, (i32, i32), String, Color)>>;
pub type Buttons<ButtonId> = Option<Vec<Button<ButtonId>>>;
type Images = Option<Vec<((i32, i32), (u32, u32), String)>>;
// Note: we intentionally avoid deriving `PartialEq`, `Debug`, or `Clone` for `Page` because
// closures stored in `PersistentElementsType` cannot implement these traits. If needed,
// custom implementations can be provided for debugging or comparison.
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

// We can clone `Page` because all of its fields implement `Clone` when closures are stored
// in `Rc`. Deriving `Clone` enables cloning of pages for caching and rendering purposes.
impl<PageId: Clone, ButtonId: Clone> Clone for Page<PageId, ButtonId>
{
    fn clone(&self) -> Self
    {
        Self { has_persistent_elements: self.has_persistent_elements.clone(), has_userinput: self.has_userinput.clone(), id: self.id.clone(), background_color: self.background_color, rects: self.rects.clone(), buttons: self.buttons.clone(), texts: self.texts.clone(), images: self.images.clone() }
    }
}
pub struct PersistentElements<PageId, ButtonId>
{
    pub id: PageId,
    pub background_color: Option<Color>,
    pub rects: Rects,
    pub buttons: Buttons<ButtonId>,
    pub texts: Texts,
    pub images: Images
}

// Similarly, implement `Clone` for `PersistentElements`.
impl<PageId: Clone, ButtonId: Clone> Clone for PersistentElements<PageId, ButtonId>
{
    fn clone(&self) -> Self
    {
        Self { id: self.id.clone(), background_color: self.background_color, rects: self.rects.clone(), buttons: self.buttons.clone(), texts: self.texts.clone(), images: self.images.clone() }
    }
}

type OptionPageInputLinked<PageId, ButtonId> = Option<Vec<Rc<dyn Fn(&mut Vec<String>) -> Page<PageId, ButtonId>>>>;
type OptionPageLinked<PageId, ButtonId> = Option<Vec<Rc<dyn Fn() -> Page<PageId, ButtonId>>>>;
type PageInputLinked<PageId, ButtonId> = Vec<(PageId, Rc<dyn Fn(&mut Vec<String>) -> Page<PageId, ButtonId>>)>;
type PageLinked<PageId, ButtonId> = Vec<(PageId, Rc<dyn Fn() -> Page<PageId, ButtonId>>)>;
pub struct PageData<PageId, ButtonId>
{
    pub vec_user_input: Vec<(PageId, ButtonId, String)>,
    pub vec_user_input_string: Vec<String>,
    pub page_history: (VecDeque<PageId>, usize),
    pub page_linked: PageLinked<PageId, ButtonId>,
    pub page_w_input_linked: PageInputLinked<PageId, ButtonId>,
    pub page_to_render: Option<Page<PageId, ButtonId>>,
    pub persistent_elements_to_render: Option<Vec<PersistentElements<PageId, ButtonId>>>,
    pub forced_persistent_elements: Option<Vec<PersistentElements<PageId, ButtonId>>>
}

// Implement `Clone` for `PageData` as well. This clones the vectors of pages and persistent elements.
impl<PageId: Clone, ButtonId: Clone> Clone for PageData<PageId, ButtonId>
{
    fn clone(&self) -> Self
    {
        Self { vec_user_input: self.vec_user_input.clone(), vec_user_input_string: self.vec_user_input_string.clone(), page_history: self.page_history.clone(), page_linked: self.page_linked.clone(), page_w_input_linked: self.page_w_input_linked.clone(), page_to_render: self.page_to_render.clone(), persistent_elements_to_render: self.persistent_elements_to_render.clone(), forced_persistent_elements: None }
    }
}
impl<PageId: Copy + Eq + Debug, ButtonId: Copy + Eq + Debug> PageData<PageId, ButtonId>
{
    /// Define PageData Default  Config
    pub fn new(app_state: &AppState<PageId, ButtonId>) -> Self
    {
        Self { vec_user_input: Vec::new(), vec_user_input_string: Vec::new(), persistent_elements_to_render: None, page_history: (VecDeque::from([app_state.current_page]), 0), page_linked: Vec::new(), page_w_input_linked: Vec::new(), page_to_render: None, forced_persistent_elements: None }
    }

    ///Link The Page With Your Determined PageId
    pub fn populate_rps_data(&mut self, option_page_linked_received: OptionPageLinked<PageId, ButtonId>, option_page_w_input_linked_received: OptionPageInputLinked<PageId, ButtonId>)
    {
        if let Some(page_linked_received) = option_page_linked_received
        {
            let mut page_linked_to_send = Vec::new();
            for page in page_linked_received
            {
                let page_id = page().id;
                page_linked_to_send.push((page_id, page));
            }
            self.page_linked = page_linked_to_send;
        };

        if let Some(page_w_input_linked_received) = option_page_w_input_linked_received
        {
            let mut page_w_input_linked_to_send = Vec::new();
            for page_w_input in page_w_input_linked_received
            {
                let page_id = page_w_input(&mut self.vec_user_input_string).id;
                page_w_input_linked_to_send.push((page_id, page_w_input));
            }
            self.page_w_input_linked = page_w_input_linked_to_send;
        };
    }

    /// Create The Page Setted In THe AppState current_page
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
                // collect persistent element instances by invoking each closure via deref
                self.persistent_elements_to_render = Some(result.iter().map(|(_, f)| f()).collect());
            }
            else
            {
                // Explicitly clear persistent elements when this page has none
                self.persistent_elements_to_render = None;
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
                // call the boxed closure via deref to construct the page
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
                    // call the closure to construct the page with the current input
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
            // call the boxed closure via deref to construct the page and inspect its input bindings
            let page = (*tuple_page.1)(&mut self.vec_user_input_string);
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
    /// Returns the topmost clickable button under the cursor,
    /// respecting visual layering (persistent fully blocks below)
    pub fn page_button_at(&self, app_state: &AppState<PageId, ButtonId>, mouse_x: f32, mouse_y: f32) -> Option<ButtonId>
    {
        let window_size = app_state.window_size;
        let stretch = app_state.stretch_mode_is_on;

        let (mx, my) = if stretch { (mouse_x * (WINDOW_DEFAULT_SCALE.0 as f32 / window_size.0 as f32), mouse_y * (WINDOW_DEFAULT_SCALE.1 as f32 / window_size.1 as f32)) } else { (mouse_x, mouse_y) };

        let inside = |r: &Rect| -> bool { mx >= r.x as f32 && mx <= (r.x + r.w) as f32 && my >= r.y as f32 && my <= (r.y + r.h) as f32 };

        // 0. forced_persistent_elements in the most top of the stack
        if let Some(persistents) = &self.forced_persistent_elements
        {
            for persistent in persistents.iter().rev()
            {
                // Buttons (clickable)
                if let Some(buttons) = &persistent.buttons
                {
                    for button in buttons.iter().rev()
                    {
                        if inside(&button.rect)
                        {
                            // If button enabled, return it
                            if button.enabled
                            {
                                return Some(button.id);
                            }
                            else
                            {
                                // Disabled button still blocks clicks below
                                return None;
                            }
                        }
                    }
                }

                // Rects (block clicks)
                if let Some(rects) = &persistent.rects
                {
                    for (_, (rect, _)) in rects.iter().rev()
                    {
                        if inside(rect)
                        {
                            return None;
                        }
                    }
                }

                // Images (block clicks)
                if let Some(images) = &persistent.images
                {
                    for (pos, size, _) in images.iter().rev()
                    {
                        if inside(&Rect::new(pos.0, pos.1, size.0, size.1))
                        {
                            return None;
                        }
                    }
                }
            }
        }


        // === 1. Persistent elements (always on top) ===
        if let Some(persistents) = &self.persistent_elements_to_render
        {
            for persistent in persistents.iter().rev()
            {
                // Buttons (clickable)
                if let Some(buttons) = &persistent.buttons
                {
                    for button in buttons.iter().rev()
                    {
                        if inside(&button.rect)
                        {
                            // If button enabled, return it
                            if button.enabled
                            {
                                return Some(button.id);
                            }
                            else
                            {
                                // Disabled button still blocks clicks below
                                return None;
                            }
                        }
                    }
                }

                // Rects (block clicks)
                if let Some(rects) = &persistent.rects
                {
                    for (_, (rect, _)) in rects.iter().rev()
                    {
                        if inside(rect)
                        {
                            return None;
                        }
                    }
                }

                // Images (block clicks)
                if let Some(images) = &persistent.images
                {
                    for (pos, size, _) in images.iter().rev()
                    {
                        if inside(&Rect::new(pos.0, pos.1, size.0, size.1))
                        {
                            return None;
                        }
                    }
                }
            }
        }

        // === 2. Page elements (below persistent) ===
        if let Some(page) = &self.page_to_render
        {
            // Buttons (clickable)
            if let Some(buttons) = &page.buttons
            {
                for button in buttons.iter().rev()
                {
                    if inside(&button.rect)
                    {
                        if button.enabled
                        {
                            return Some(button.id);
                        }
                        else
                        {
                            return None;
                        }
                    }
                }
            }

            // Rects (block clicks)
            if let Some(rects) = &page.rects
            {
                for (_, (rect, _)) in rects.iter().rev()
                {
                    if inside(rect)
                    {
                        return None;
                    }
                }
            }

            // Images (block clicks)
            if let Some(images) = &page.images
            {
                for (pos, size, _) in images.iter().rev()
                {
                    if inside(&Rect::new(pos.0, pos.1, size.0, size.1))
                    {
                        return None;
                    }
                }
            }
        }

        None
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
