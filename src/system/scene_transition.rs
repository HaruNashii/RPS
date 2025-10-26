use std::time::Instant;





/// Scene transition effect types.
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionType 
{
    None,
    Fade(f32),
    Slide(f32, u8, i32),
}

/// Manages active scene transitions (fade or slide).
#[derive(Debug, Clone, PartialEq)]
pub struct SceneTransition<PageId> 
{
    pub active: bool,
    pub progress: f32,
    pub transition_type: TransitionType,
    pub is_second_stage: bool,
    pub has_switched: bool,
    pub start_time: Instant,
    pub duration_ms: u32,
    pub next_page: Option<PageId>,
}





impl<PageId: Copy + Eq> SceneTransition<PageId> 
{
    /// Creates a new transition of a given type and duration.
    pub fn new(transition_type: TransitionType, duration_ms: u32, next_page: Option<PageId>) -> Self 
    {
        Self 
        {
            active: true,
            progress: 0.0,
            transition_type,
            is_second_stage: false,
            has_switched: false,
            start_time: Instant::now(),
            duration_ms,
            next_page,
        }
    }

    /// Update progress and return `true` when finished.
    /// The renderer uses this to know when to clear the transition.
    pub fn update(&mut self) -> bool 
    {
        if !self.active { return true; }
        let elapsed = self.start_time.elapsed().as_millis() as f32;

        match self.transition_type 
        {
            TransitionType::Fade(_) => 
            {
                let half = (self.duration_ms as f32) / 2.0;
                self.progress = (elapsed / half).clamp(0.0, 1.0);

                if !self.is_second_stage 
                {
                    self.transition_type = TransitionType::Fade(self.progress);
                    if self.progress >= 1.0 
                    {
                        self.is_second_stage = true;
                        self.start_time = Instant::now();
                        self.progress = 0.0;
                    }
                } 
                else 
                {
                    self.transition_type = TransitionType::Fade(1.0 - self.progress);
                    if self.progress >= 1.0 
                    {
                        self.active = false;
                        return true;
                    }
                }
            }

            TransitionType::Slide(_, move_pos, speed) => 
            {
                // Single continuous stage for slides (0..1)
                let total = self.duration_ms as f32;
                self.progress = (elapsed / total).clamp(0.0, 1.0);
                self.transition_type = TransitionType::Slide(self.progress, move_pos, speed);

                // Slide uses "second stage" semantics for renderer compatibility
                if !self.is_second_stage 
                {
                    self.is_second_stage = true;
                    self.start_time = Instant::now();
                }

                if self.progress >= 1.0 
                {
                    self.active = false;
                    return true;
                }
            }

            TransitionType::None => 
            {
                self.active = false;
                return true;
            }
        }

        false
    }
}

