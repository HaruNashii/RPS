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





pub trait MouseInput { fn handle_mouse_input(&mut self) -> (Option<usize>, Option<usize>); }
impl MouseInput for ((Option<&Vec<(bool, Color, Rect, u16)>>, &Vec<(bool, Color, Rect, u16)>), &mut EventPump, (u32, u32))
{
    fn handle_mouse_input(&mut self) -> (Option<usize>, Option<usize>)
    {
        let mouse_state = EventPump::mouse_state(self.1);
        let x = mouse_state.x();
        let y = mouse_state.y();
        let mut button_being_hovered = None;
        let mut button_clicked = None;

        let x_scaled = x * (1920.00 / self.2.0 as f32);
        let y_scaled = y * (1080.00 / self.2.1 as f32);

        if let Some(result) = self.0.0
        {
            for object in result
            {
                if x_scaled >= object.2.x as f32 && x_scaled <= (object.2.x + object.2.w) as f32 && y_scaled >= object.2.y as f32 && y_scaled <= (object.2.y + object.2.h) as f32
                {
                    button_being_hovered = Some(object.3 as usize); 
                }
            }
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
        
        (button_being_hovered, button_clicked)
    }
}





pub trait KeyboardInput { fn handle_keyboard_input(&mut self) -> String; }
impl KeyboardInput for &mut EventPump
{
    fn handle_keyboard_input(&mut self) -> String
    {   
        let mut is_on_write_mode: (bool, Option<u16>) = (false, None);
        let mut user_input = String::new();

        for event in self.poll_iter() 
        {
            match event 
            {
                Event::TextInput{text, .. } =>
                {
                    if is_on_write_mode.0 { user_input.push_str(&text) };
                }

                Event::KeyDown{keycode: Some(Keycode::Backspace), .. } =>
                {
                    if is_on_write_mode.0 && !user_input.is_empty() { user_input.pop(); };
                }

                Event::KeyDown{keycode: Some(Keycode::Return), .. } =>
                {
                    println!("Return Pressed!!!");
                }

                Event::KeyDown{keycode: Some(Keycode::Escape), .. } => 
                {
                    if is_on_write_mode.0
                    {
                        user_input.clear();
                        is_on_write_mode = (false, None);
                    };
                }

                Event::Quit { .. } => 
                { 
                    exit(0); 
                }

                _ => {}
            }
        }

        user_input
    }
}
