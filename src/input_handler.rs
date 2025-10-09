use std::process::exit;
use sdl3::
{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::Rect,
    EventPump
};
use crate::pages::COLOR_CHANGE_WHEN_SELECTED;






pub trait ChangeColors
{
   fn button_change_color_when_hovered(self) -> Vec<(bool, Color, Rect, u16)>;
}

impl ChangeColors for (&Vec<(bool, Color, Rect, u16)>, Option<usize>)
{
    fn button_change_color_when_hovered(self) -> Vec<(bool, Color, Rect, u16)>
    {
        let mut vec_of_buttons = self.0.clone();

        if let Some(button_being_hovered) = self.1 
        {
            for button in &mut vec_of_buttons
            {
                if button_being_hovered as u16 == button.3
                {
                    if (button.1.r as i32 - COLOR_CHANGE_WHEN_SELECTED.0 as i32) > 1 { button.1.r -= COLOR_CHANGE_WHEN_SELECTED.0 } else { button.1.r = 0 };
                    if (button.1.g as i32 - COLOR_CHANGE_WHEN_SELECTED.1 as i32) > 1 { button.1.g -= COLOR_CHANGE_WHEN_SELECTED.1 } else { button.1.g = 0 };
                    if (button.1.b as i32 - COLOR_CHANGE_WHEN_SELECTED.2 as i32) > 1 { button.1.b -= COLOR_CHANGE_WHEN_SELECTED.2 } else { button.1.b = 0 };
                };
            };
        }

        vec_of_buttons
    }
}





type MouseInputReturn = (Option<usize>, (Option<Vec<(bool, Color, Rect, u16)>>, Vec<(bool, Color, Rect, u16)>));
pub trait MouseInput { fn handle_mouse_input(&mut self) -> MouseInputReturn; }
impl MouseInput for ((Option<&Vec<(bool, Color, Rect, u16)>>, &Vec<(bool, Color, Rect, u16)>), &mut EventPump, (u32, u32))
{
    fn handle_mouse_input(&mut self) -> (Option<usize>, (Option<Vec<(bool, Color, Rect, u16)>>, Vec<(bool, Color, Rect, u16)>))
    {
        let mouse_state = EventPump::mouse_state(self.1);
        let x = mouse_state.x();
        let y = mouse_state.y();
        let mut button_being_hovered = None;
        let mut button_clicked = None;

        let x_scaled = x * (1920.00 / self.2.0 as f32);
        let y_scaled = y * (1080.00 / self.2.1 as f32);

        if let Some(page_buttons) = self.0.0 
        {
            for buttons in page_buttons
            {
                if x_scaled >= buttons.2.x as f32 && x_scaled <= (buttons.2.x + buttons.2.w) as f32 && y_scaled >= buttons.2.y as f32 && y_scaled <= (buttons.2.y + buttons.2.h) as f32
                {
                    button_being_hovered = Some(buttons.3 as usize); 
                }
            };
        };

        for object in self.0.1
        {
            if x_scaled >= object.2.x as f32 && x_scaled <= (object.2.x + object.2.w) as f32 && y_scaled >= object.2.y as f32 && y_scaled <= (object.2.y + object.2.h) as f32
            {
                button_being_hovered = Some(object.3 as usize); 
            }
        };

        for event in self.1.poll_iter() 
        {
            match event 
            {
                Event::MouseButtonDown {mouse_btn: MouseButton::Left, ..} =>
                {
                    if let Some(result) = button_being_hovered {button_clicked = Some(result)};
                }

                Event::Quit { .. } => 
                { 
                    exit(0); 
                }

                _ => {}
            }
        }

        let mut new_persistent_page_button_colors = None;
        if let Some(result) = self.0.0 { new_persistent_page_button_colors = Some((result, button_being_hovered).button_change_color_when_hovered()); };
        let new_button_colors = (self.0.1, button_being_hovered).button_change_color_when_hovered();
        
        (button_clicked, (new_persistent_page_button_colors, new_button_colors))
    }
}





pub trait KeyboardInput { fn handle_keyboard_input(&mut self) -> (String, bool); }
impl KeyboardInput for (&mut EventPump, String, bool)
{
    fn handle_keyboard_input(&mut self) -> (String, bool)
    {   
        println!("function executed");
        for event in self.0.poll_iter() 
        {
            match event 
            {
                Event::TextInput{text, .. } =>
                {
                    println!("any text pressed");
                    self.1.push_str(&text);
                }

                Event::KeyDown{keycode: Some(Keycode::Backspace), .. } =>
                {
                    println!("any text pressed");
                    if !self.1.is_empty() { self.1.pop(); };
                }

                Event::KeyDown{keycode: Some(Keycode::Return), .. } =>
                {
                    println!("any text pressed");
                    self.2 = false;
                }

                Event::KeyDown{keycode: Some(Keycode::Escape), .. } => 
                {
                    println!("any text pressed");
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
