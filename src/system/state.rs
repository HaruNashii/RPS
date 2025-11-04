use crate::{
    system::page_system::PageData,
    system::scene_transition::{SceneTransition, TransitionType}
};
use std::fmt::Debug;

/// Global application state that holds page navigation,
/// transitions, and user input tracking.
#[derive(Debug, Clone)]
pub struct AppState<PageId, ButtonId>
{
    pub current_page: PageId,
    pub scene_transition: Option<SceneTransition<PageId>>,
    pub current_transition_type: Option<TransitionType>,
    pub window_size: (u32, u32),
    pub capturing_input: (bool, Option<ButtonId>)
}

impl<PageId: Copy + Eq + Debug, ButtonId: Copy + Eq + Debug> AppState<PageId, ButtonId>
{
    /// Create a new app state with a starting page.
    pub fn new(start_page: PageId, window_size: (u32, u32)) -> Self
    {
        Self { current_page: start_page, scene_transition: None, current_transition_type: None, window_size, capturing_input: (false, None) }
    }

    /// Change to a new page, optionally triggering a transition.
    pub fn change_current_page(&mut self, page_data: &mut PageData<PageId, ButtonId>, next_page: PageId, button_id: &ButtonId)
    {
        if next_page == self.current_page
        {
            return;
        }
        if let Some(page_to_render) = page_data.page_to_render.clone()
            && let Some(vec_of_buttons) = page_to_render.buttons
            && let Some(received_button) = vec_of_buttons.iter().find(|button| button.id == *button_id && button.has_transition.is_some())
            && let Some(transition) = received_button.has_transition
        {
            // Start a new SceneTransition
            self.scene_transition = Some(SceneTransition::new(transition, 500, Some(next_page)));
        }
        else if let Some(vec_persistent_elements_to_render) = page_data.persistent_elements_to_render.clone()
        {
            for p in vec_persistent_elements_to_render
            {
                if let Some(vec_of_buttons) = p.buttons
                    && let Some(received_button) = vec_of_buttons.iter().find(|button| button.id == *button_id && button.has_transition.is_some())
                    && let Some(transition) = received_button.has_transition
                {
                    // Start a new SceneTransition
                    self.scene_transition = Some(SceneTransition::new(transition, 500, Some(next_page)));
                }
                else
                {
                    // No transition requested → just switch directly.
                    self.current_page = next_page;
                }
            }
        }
        else
        {
            // No transition requested → just switch directly.
            self.current_page = next_page;
        }

        // Track history if your PageData keeps navigation stack
        if !page_data.page_history.0.contains(&next_page)
        {
            page_data.page_history.0.push_back(next_page);
        }
        page_data.page_history.1 = page_data.page_history.0.len().saturating_sub(1);
    }

    /// Returns whether a scene transition is currently active.
    pub fn is_transition_active(&self) -> bool
    {
        self.scene_transition.as_ref().is_some_and(|t| t.active)
    }

    /// Update window size (e.g., on resize events)
    pub fn update_window_size(&mut self, width: u32, height: u32)
    {
        self.window_size = (width, height);
    }

    /// Begin capturing user input for a specific button ID
    pub fn begin_capturing_input(&mut self, button: ButtonId)
    {
        self.capturing_input = (true, Some(button));
    }

    /// Stop capturing user input
    pub fn stop_capturing_input(&mut self)
    {
        self.capturing_input = (false, None);
    }

    /// Check if currently capturing input
    pub fn is_capturing(&self) -> bool
    {
        self.capturing_input.0
    }
}
