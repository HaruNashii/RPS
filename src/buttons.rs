use sdl3::{pixels::Color, rect::Rect};
use crate::{pages::COLOR_CHANGE_WHEN_SELECTED};





pub static mut ALLOW_QUERY: bool = true;





pub trait ChangeColors
{
   fn button_change_color_when_hovered(self) -> Vec<(bool, Color, Rect, u16)>;
}

impl ChangeColors for (Vec<(bool, Color, Rect, u16)>, Option<usize>)
{
    fn button_change_color_when_hovered(mut self) -> Vec<(bool, Color, Rect, u16)>
    {
        if let Some(button_being_hovered) = self.1 
        {
            for button in &mut self.0
            {
                if button_being_hovered as u16 == button.3
                {
                    if (button.1.r as i32 - COLOR_CHANGE_WHEN_SELECTED.0 as i32) > 1 { button.1.r -= COLOR_CHANGE_WHEN_SELECTED.0 } else { button.1.r = 0 };
                    if (button.1.g as i32 - COLOR_CHANGE_WHEN_SELECTED.1 as i32) > 1 { button.1.g -= COLOR_CHANGE_WHEN_SELECTED.1 } else { button.1.g = 0 };
                    if (button.1.b as i32 - COLOR_CHANGE_WHEN_SELECTED.2 as i32) > 1 { button.1.b -= COLOR_CHANGE_WHEN_SELECTED.2 } else { button.1.b = 0 };
                };
            };
        }

        self.0
    }
}





pub fn button_action(button_clicked: Option<usize>, page_to_render: &mut u8, mut can_get_user_input: bool) -> bool
{
    match button_clicked
    {
        Some(1) =>
        {
            //PAGE 1 BUTTON (PERSISTENT PAGE)
            *page_to_render = 1;
        }

        Some(2) =>
        {
            //PAGE 2 BUTTON (PERSISTENT PAGE)
            *page_to_render = 2;
        }

        Some(3) => 
        {
            // PAGE 1 BUTTON (PAGE 1)
            if !can_get_user_input { can_get_user_input = true };
        }

        Some(4) => 
        {
            //PAGE 2 TO SUBPAGE_PAGE2 BUTTON (PAGE 2)
            *page_to_render = 3;
        }
            
        Some(5) => 
        {
            //SUBPAGE_PAGE2 TO PAGE 2 BUTTON (PAGE 3)
            *page_to_render = 2;
        }

        Some(6) =>
        {
            // PAGE 2 BUTTON (PAGE 2)
            println!("button 6 pressed");
            if !can_get_user_input { can_get_user_input = true };
        }
        
        _=> {},
    }

   can_get_user_input 
}
