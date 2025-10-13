use sdl3::
{
    EventPump,
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton
};





#[derive(Debug, Clone)]
pub enum InputEvent 
{
    Click(f32, f32),
    Text(String),    
    Backspace,       
    Submit,          
    Quit,
    None,
}
pub struct InputHandler;
impl InputHandler 
{
    pub fn poll(&self, event_pump: &mut EventPump) -> InputEvent 
    {
        for event in event_pump.poll_iter() 
        {
            match event 
            {
                Event::Quit { .. } => return InputEvent::Quit,
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => return InputEvent::Click(x, y),
                Event::TextInput { text, .. } => return InputEvent::Text(text),
                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => return InputEvent::Backspace,
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => return InputEvent::Submit,
                _ => {}
            }
        }
        InputEvent::None
    }
}
