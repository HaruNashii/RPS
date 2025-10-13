use sdl3::
{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    EventPump
};
use crate::ui::pages::ButtonId;
use crate::system::state::AppState;





#[derive(Debug, Clone)]
pub enum InputEvent 
{
    Click(ButtonId),
    Text(String),
    Submit,
    Quit,
    None,
}

pub struct InputHandler;
impl InputHandler 
{
    pub fn poll(&self, state: &mut AppState, event_pump: &mut EventPump) -> InputEvent 
    {
        for event in event_pump.poll_iter() 
        {
            match event 
            {
                Event::Quit { .. } => return InputEvent::Quit,

                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => 
                {
                    if let Some(id) = state.page_at(x, y) 
                    {
                        return InputEvent::Click(id);
                    }
                }

                Event::TextInput { text, .. } => 
                {
                    if state.capturing_input 
                    {
                        let current_page = state.current_page;
                        let vec_user_input = &state.vec_user_input;
                        let mut new_vec = vec_user_input.clone();

                        for (index, user_input) in vec_user_input.iter().enumerate()
                        {
                            if user_input.1 == current_page
                            {
                                new_vec[index].0.push_str(&text)   
                            }
                        }
                        state.vec_user_input = new_vec;
                        return InputEvent::Text(text);
                    }
                }

                Event::KeyDown { keycode: Some(Keycode::Return), .. } => 
                {
                    if state.capturing_input 
                    {
                        state.submit_input();
                        return InputEvent::Submit;
                    }
                }

                _ => {}
            }
        }
        InputEvent::None
    }
}

