use sdl3::
{
    pixels::Color,
    rect::Rect
};
use crate::{ButtonId, PageId};
use crate::system::window::WINDOW_DEFAULT_SCALE;





type Rects = Option<Vec<(Color, (Rect, i32))>>;
type Buttons = Option<Vec<Button>>;
type Texts = Option<Vec<(f64, (i32, i32), String, Color)>>;
type Images = Option<Vec<((i32, i32), (u32, u32), String)>>;
#[derive(Debug, Clone)]
pub struct Page 
{
    pub has_persistant_page: bool,
    pub id: PageId,
    pub background_color: Option<Color>,
    pub rects: Rects,
    pub buttons: Buttons,
    pub texts: Texts,
    pub images: Images,
}





#[derive(Clone, Copy, Debug)]
pub struct Button 
{
    pub enabled: bool,
    pub color: Color,
    pub rect: Rect,
    pub radius: i32,
    pub id: ButtonId,
}






impl Page 
{
    pub fn button_at(&self, mouse_pos_x: f32, mouse_pos_y: f32, window_size: (u32, u32)) -> Option<ButtonId> 
    {
        if let Some(vec_buttons) = &self.buttons 
        {
            let x_scaled = mouse_pos_x * (WINDOW_DEFAULT_SCALE.0 as f32 / window_size.0 as f32);
            let y_scaled = mouse_pos_y * (WINDOW_DEFAULT_SCALE.1 as f32 / window_size.1 as f32);
            for button in vec_buttons 
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
