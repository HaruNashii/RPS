use std::fmt::Debug;
use crate::system::{page_system::PageData, window::WINDOW_DEFAULT_SCALE};





/// Global application state that holds the state and logic data.
#[derive(PartialEq, Debug, Clone)]
pub struct AppState<PageId, ButtonId>
{
    pub current_page: PageId,
    pub capturing_input: (bool, Option<ButtonId>),
    pub window_size: (u32, u32),
}
impl<PageId: Copy + Eq + Debug, ButtonId: Copy + Eq + Debug> AppState<PageId, ButtonId>
{
    /// Create The App State
    pub fn new(page_id: PageId) -> Self { Self { current_page: page_id, capturing_input: (false, None), window_size: (WINDOW_DEFAULT_SCALE.0, WINDOW_DEFAULT_SCALE.1), } }
    pub fn change_current_page(&mut self, app_data: &mut PageData<PageId,ButtonId>, page_to_change: PageId) { if page_to_change != self.current_page { self.current_page = page_to_change; app_data.page_history.0.push_back(self.current_page); app_data.page_history.1 += 1; } }
    pub fn update_window_size(&mut self, received_window_size: (u32, u32)) { self.window_size = received_window_size}
}
