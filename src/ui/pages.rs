use std::env;
use crate::
{
    misc::center_elements::get_center,
    system::window::WINDOW_DEFAULT_SCALE,
    ui::style::{BACKGROUND_COLOR, BLACK_COLOR, ORANGE_COLOR, PINK_COLOR, PURPLE_COLOR, RED_COLOR, SUBTEXT_COLOR, TEXT_COLOR},
};
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
    pub fn persistent_page() -> Self 
    {
        //===================== variables =========================
        let padding_x = 200;
        let window_center = get_center((200, 75), WINDOW_DEFAULT_SCALE);

        //===================== rects =========================
        let all_rects = vec!
        [
            (BLACK_COLOR, (Rect::new(0, 0, 1920, 100), 0))
        ];

        //===================== buttons =========================
        let all_buttons = vec!
        [
            Button { enabled: true, color: PINK_COLOR, rect: Rect::new(window_center.pos_x - padding_x, 10, window_center.w, window_center.h), radius: 5, id: ButtonId::ButtonPage1 },
            Button { enabled: true, color: PINK_COLOR, rect: Rect::new(window_center.pos_x + padding_x, 10, window_center.w, window_center.h), radius: 5, id: ButtonId::ButtonPage2 }
        ];

        //===================== texts =========================
        let all_text = vec!
        [
            //page_1 button text
            (17.0, (all_buttons[0].rect.x + 9, all_buttons[0].rect.y + 24), "Page 1".to_string(), TEXT_COLOR),
            //page_2 button text
            (17.0, (all_buttons[1].rect.x + 9, all_buttons[1].rect.y + 24), "Page 2".to_string(), TEXT_COLOR),
        ];

        //===================== images =========================
        let all_images = vec!
        [
            ((10, 10), (50, 50), format!("{}/.cache/page_system/example_1.jpg", env::home_dir().unwrap().display()))
        ];

        //===================== page creation =========================
        Self { has_persistant_page: false, id: PageId::Persistent, background_color: None, rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_text), images: Some(all_images) }
    }

    pub fn page_1(user_input: Vec<String>) -> Self 
    {
        //===================== variables =========================
        let padding_y = 20;
        let red_rect_data = get_center((200, 200), WINDOW_DEFAULT_SCALE);
        let orange_rect_data = get_center((800, 200), WINDOW_DEFAULT_SCALE);
        let purple_button_data = get_center((600, 100), WINDOW_DEFAULT_SCALE);

        //===================== rects =========================
        let all_rects = vec!
        [
            (RED_COLOR, (Rect::new(red_rect_data.pos_x, red_rect_data.pos_y + (orange_rect_data.h as i32 + padding_y), red_rect_data.w, red_rect_data.h), 100)),
            (ORANGE_COLOR, (Rect::new(orange_rect_data.pos_x, orange_rect_data.pos_y, orange_rect_data.w, orange_rect_data.h), 0))
        ];

        //===================== buttons =========================
        let all_buttons = vec!
        [
            Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(purple_button_data.pos_x, purple_button_data.pos_y - (orange_rect_data.h as i32 - padding_y),       purple_button_data.w, purple_button_data.h), radius: 20, id: ButtonId::ButtonPurpleInputStartPage1 },
            Button { enabled: true, color: RED_COLOR,    rect: Rect::new(purple_button_data.pos_x, all_rects[0].1.0.y + all_rects[0].1.0.h + padding_y, purple_button_data.w, purple_button_data.h), radius: 0, id: ButtonId::ButtonRedInputStartPage1 }
        ];

        //===================== texts =========================
        let all_text = vec!
        [
            (18.0, (all_rects[1].1.0.x + 165, all_rects[1].1.0.y + 86), "Random Orange Rectangle, Because I Can :)".to_string(), SUBTEXT_COLOR), 
            (18.0, (all_buttons[0].rect.x + 75, all_buttons[0].rect.y - 25), "Click the Button To Start Getting Input".to_string(), SUBTEXT_COLOR),
            (25.0, (all_buttons[0].rect.x + 15, all_buttons[0].rect.y + 35), user_input[0].clone(), BLACK_COLOR),
            (25.0, (all_buttons[1].rect.x + 15, all_buttons[1].rect.y + 35), user_input[1].clone(), BLACK_COLOR)
        ];

        //===================== page creation =========================
        Self { has_persistant_page: true, id: PageId::Page1, background_color: Some(BACKGROUND_COLOR), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_text), images: None }
    }

    pub fn page_2(user_input: Vec<String>) -> Self 
    {
        //===================== variables =========================
        let get_input_button_data = get_center((500, 100), WINDOW_DEFAULT_SCALE);

        //===================== buttons =========================
        let all_buttons = vec!
        [
            Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(100, 150, 235, 40), radius: 20, id: ButtonId::ButtonSubPage },
            Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(get_input_button_data.pos_x, get_input_button_data.pos_y, get_input_button_data.w as u32, get_input_button_data.h as u32), radius: 20, id: ButtonId::ButtonPurpleInputStartPage2 }
        ];

        //===================== texts =========================
        let all_text = vec!
        [
            (18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "Go To subpage_page2".to_string(), TEXT_COLOR),
            (18.0, (all_buttons[1].rect.x + 10, all_buttons[1].rect.y + 7), user_input[0].clone(), TEXT_COLOR)
        ];

        //===================== page creation =========================
        Self { has_persistant_page: true, id: PageId::Page2, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }
    }

    pub fn subpage_page2() -> Self 
    {
        //===================== buttons =========================
        let all_buttons = vec!
        [
            Button { enabled: true, color: PINK_COLOR, rect: Rect::new(20, 20, 50, 40), radius: 0, id: ButtonId::ButtonBack }
        ];

        //===================== texts =========================
        let all_text = vec!
        [
            (18.0, (950, 400), "Random Text, Because I Can :)".to_string(), SUBTEXT_COLOR),
            (18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "<-".to_string(), TEXT_COLOR)
        ];

        //===================== images =========================
        let all_images = vec!
        [
            ((500, 500), (300, 300), format!("{}/.cache/page_system/example_2.jpg", env::home_dir().unwrap().display()))
        ];

        //===================== page creation =========================
        Self { has_persistant_page: false, id: PageId::Page2SubPage, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: Some(all_images) }
    }
}
