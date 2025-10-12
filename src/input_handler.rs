use std::process::exit;
use sdl3::
{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    EventPump
};
use crate::pages::{Button, ButtonId, COLOR_CHANGE_WHEN_SELECTED};






pub trait ChangeColors
{
   fn button_change_color_when_hovered(self) -> Option<Vec<Button>>;
}

impl ChangeColors for (Option<Vec<Button>>, Option<ButtonId>)
{
    fn button_change_color_when_hovered(mut self) -> Option<Vec<Button>>
    {
        if let Some(button_hovered) = self.1 && self.0.is_some()
        {
            for button in self.0.as_mut().unwrap()
            {
                if button_hovered as usize == button.id as usize
                {
                    if (button.color.r as i32 - COLOR_CHANGE_WHEN_SELECTED.0 as i32) > 1 { button.color.r -= COLOR_CHANGE_WHEN_SELECTED.0 } else { button.color.r = 0 };
                    if (button.color.g as i32 - COLOR_CHANGE_WHEN_SELECTED.1 as i32) > 1 { button.color.g -= COLOR_CHANGE_WHEN_SELECTED.1 } else { button.color.g = 0 };
                    if (button.color.b as i32 - COLOR_CHANGE_WHEN_SELECTED.2 as i32) > 1 { button.color.b -= COLOR_CHANGE_WHEN_SELECTED.2 } else { button.color.b = 0 };
                };
            }
        }; 

        self.0
    }
}





type MouseInputReturn = (Option<ButtonId>, (Option<Vec<Button>>, Vec<Button>));
pub trait MouseInput { fn handle_mouse_input(&mut self) -> MouseInputReturn; }
impl MouseInput for ((&Option<Vec<Button>>, &Option<Vec<Button>>), &mut EventPump, (u32, u32))
{
    fn handle_mouse_input(&mut self) -> MouseInputReturn
    {
        let mouse_state = EventPump::mouse_state(self.1);
        let x = mouse_state.x();
        let y = mouse_state.y();
        let mut button_being_hovered = None;
        let mut button_clicked = None;

        let x_scaled = x * (1920.00 / self.2.0 as f32);
        let y_scaled = y * (1080.00 / self.2.1 as f32);

        if let Some(persistent_page_buttons) = self.0.0
        {
            for buttons in persistent_page_buttons
            {
                if x_scaled >= buttons.rect.x as f32 && x_scaled <= (buttons.rect.x + buttons.rect.w) as f32 && y_scaled >= buttons.rect.y as f32 && y_scaled <= (buttons.rect.y + buttons.rect.h) as f32
                {
                    button_being_hovered = Some(buttons.id); 
                }
            };
        };
        
        if let Some(page_buttons) = self.0.1
        {
            for buttons in page_buttons
            {
                if x_scaled >= buttons.rect.x as f32 && x_scaled <= (buttons.rect.x + buttons.rect.w) as f32 && y_scaled >= buttons.rect.y as f32 && y_scaled <= (buttons.rect.y + buttons.rect.h) as f32
                {
                    button_being_hovered = Some(buttons.id); 
                }
            };
        }

        for event in self.1.poll_iter() 
        {
            match event 
            {
                Event::MouseButtonDown {mouse_btn: MouseButton::Left, ..} =>
                {
                    if let Some(button_hovered) = button_being_hovered {button_clicked = Some(button_hovered)};
                }

                Event::Quit { .. } => 
                { 
                    exit(0); 
                }

                _ => {}
            }
        }


        let new_persistent_page_button_colors = (self.0.0.clone(), button_being_hovered).button_change_color_when_hovered();
        let new_button_colors = (self.0.1.clone(), button_being_hovered).button_change_color_when_hovered();
        
        (button_clicked, (new_persistent_page_button_colors, new_button_colors.unwrap()))
    }
}





pub trait KeyboardInput { fn handle_keyboard_input(&mut self) -> (String, bool); }
impl KeyboardInput for (&mut EventPump, String, bool)
{
    fn handle_keyboard_input(&mut self) -> (String, bool)
    {   
        for event in self.0.poll_iter() 
        {
            match event 
            {
                Event::TextInput{text, .. } =>
                {
                    self.1.push_str(&text);
                }

                Event::KeyDown{keycode: Some(Keycode::Backspace), .. } =>
                {
                    if !self.1.is_empty() { self.1.pop(); };
                }

                Event::KeyDown{keycode: Some(Keycode::Return), .. } =>
                {
                    self.2 = false;
                }

                Event::KeyDown{keycode: Some(Keycode::Escape), .. } => 
                {
                    self.2 = false;
                }

                Event::Quit { .. } => 
                { 
                    exit(0); 
                }

                _ => {}
            }
        }
    
        (self.1.clone(), self.2)
    }
}
