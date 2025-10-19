use sdl3::
{
    pixels::Color,
    rect::Rect
};
use crate::system::window::WINDOW_DEFAULT_SCALE;





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

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Button<ButtonId>
{
    pub enabled: bool,
    pub color: Color,
    pub rect: Rect,
    pub radius: i32,
    pub id: ButtonId,
}

impl<ButtonId> Button<ButtonId> where ButtonId: Copy + Eq,
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
