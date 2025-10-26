use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionType 
{
    None,
    Fade(f32),
    Slide(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneTransition<PageId> 
{
    pub active: bool,
    pub progress: f32,
    pub transition_type: TransitionType,
    pub duration_ms: u32,
    pub start_time: Instant,
    pub next_page: Option<PageId>,
    pub has_switched: bool,
    pub is_second_stage: bool, // simple flag for two-stage
}

impl<PageId: Copy + Eq> SceneTransition<PageId>
{
    pub fn new(transition_type: TransitionType, duration_ms: u32, next_page: Option<PageId>) -> Self 
    {
        Self 
        { 
            active: true, 
            progress: 0.0, 
            transition_type, 
            duration_ms, 
            start_time: Instant::now(), 
            next_page, 
            has_switched: false,
            is_second_stage: false 
        } 
    }

    pub fn update(&mut self) -> bool 
    {
        let elapsed = self.start_time.elapsed().as_millis() as f32;
        let stage_duration = self.duration_ms as f32 / 2.0;
        self.progress = (elapsed / stage_duration).min(1.0);

        if !self.is_second_stage 
        {
            // first stage → fade out
            if let TransitionType::Fade(_) = self.transition_type 
            {
                self.transition_type = TransitionType::Fade(self.progress);
            }

            // when first half ends, switch page
            if self.progress >= 1.0 
            {
                self.is_second_stage = true;
                self.start_time = Instant::now();
                self.progress = 0.0;
            }
        } 
        else 
        {
            // second stage → fade in
            if let TransitionType::Fade(_) = self.transition_type 
            {
                self.transition_type = TransitionType::Fade(1.0 - self.progress);
            }

            if self.progress >= 1.0 
            {
                self.active = false;
                return true;
            }
        }

        false
    }
}

