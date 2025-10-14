use sdl3::
{
    pixels::Color,
    rect::Rect
};





type Rects = Option<Vec<(Color, (Rect, i32))>>;
type Buttons = Option<Vec<Button>>;
type Texts = Option<Vec<(f64, (i32, i32), String, Color)>>;
type Images = Option<Vec<((i32, i32), (u32, u32), String)>>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageId 
{
    Persistent,
    Page1,
    Page2,
    Page2SubPage,
}
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
#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(usize)]
pub enum ButtonId 
{
    ButtonPage1,
    ButtonPage2,
    ButtonPurpleInputStartPage1,
    ButtonRedInputStartPage1,
    ButtonPurpleInputStartPage2,
    ButtonSubPage,
    ButtonBack,
}






impl Page 
{
    pub fn button_at(&self, mouse_pos_x: f32, mouse_pos_y: f32, window_size: (u32, u32)) -> Option<ButtonId> 
    {
        if let Some(vec_buttons) = &self.buttons 
        {
            let x_scaled = mouse_pos_x * (1920.00 / window_size.0 as f32);
            let y_scaled = mouse_pos_y * (1080.00 / window_size.1 as f32);
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

    pub fn create_from_id(id: PageId, option_user_input: Option<Vec<(&String, &PageId)>>) -> Self 
    {
        match id 
        {
            PageId::Persistent => Self::persistent_page(),
            PageId::Page1 => 
            {
                let mut vec_string_to_send = Vec::new();
                if let Some(vec_user_input) = option_user_input
                {
                    for user_input in vec_user_input
                    {
                        vec_string_to_send.push(user_input.0.to_string());
                    }
                }
                else 
                {
                    println!("vec_user_input not provided, while the page need user_input, sending empty vec to prevent crash, but please fix it");
                    vec_string_to_send.push(String::new());
                };

                Self::page_1(vec_string_to_send)
            }
            PageId::Page2 => 
            {
                let mut vec_string_to_send = Vec::new();
                if let Some(vec_user_input) = option_user_input
                {
                    for user_input in vec_user_input
                    {
                        vec_string_to_send.push(user_input.0.to_string());
                    }
                }
                else 
                {
                    println!("vec_user_input not provided, while the page need user_input, sending empty vec to prevent crash, but please fix it");
                    vec_string_to_send.push(String::new());
                };
                Self::page_2(vec_string_to_send)
            }
            PageId::Page2SubPage => Self::subpage_page2(),
        }
    }
}
