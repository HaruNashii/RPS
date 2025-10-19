use crate::system::window::WINDOW_DEFAULT_SCALE;





/// Global application state that holds the state and logic data.
pub struct AppState<PageId, ButtonId>
{
    pub current_page: (PageId, bool),
    pub capturing_input: (bool, Option<ButtonId>),
    pub window_size: (u32, u32),
}
impl<PageId: Copy + Eq, ButtonId: Copy + Eq> AppState<PageId, ButtonId>
{
    /// Create The App State
    pub fn new(page_id: PageId, has_user_input: bool) -> Self { Self { current_page: (page_id, has_user_input), capturing_input: (false, None), window_size: (WINDOW_DEFAULT_SCALE.0, WINDOW_DEFAULT_SCALE.1), } }
    pub fn update_window_size(&mut self, received_window_size: (u32, u32)) { self.window_size = received_window_size}
}
