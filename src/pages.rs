use std::env;
use sdl3::
{
    pixels::Color, 
    rect::Rect,
};
use crate::window::WINDOW_DEFAULT_SCALE;





struct RectCenterPos { pos_y: i32, pos_x: i32, w: u32, h: u32}
fn get_center(rect_size: (i32, i32), window_pos: (u32, u32)) -> RectCenterPos
{
    let new_pos = ((window_pos.0 as i32 / 2) - (rect_size.0 / 2), (window_pos.1 as i32 / 2) - (rect_size.1 / 2));

    RectCenterPos
    {
        pos_x: new_pos.0,
        pos_y: new_pos.1,
        w: rect_size.0 as u32,
        h: rect_size.1 as u32,
    }
}





type Rects = Option<Vec<(Color, (Rect, i32))>>;
type Buttons = Option<Vec<(bool, Color, (Rect, i32), u16)>>;
type Texts = Option<Vec<(f64, (i32, i32), String, Color)>>;
type Images = Option<Vec<((i32, i32), (u32, u32), String)>>;
pub struct Page
{
    pub background_color: Option<Color>,
    pub rects:   Rects,
    pub buttons: Buttons,
    pub texts:   Texts,
    pub images:  Images,
}





pub const COLOR_CHANGE_WHEN_SELECTED: (u8, u8, u8) = (25, 25, 25);
const BACKGROUND_COLOR: Color = Color::RGB(30,  30,  46);
const TEXT_COLOR:       Color = Color::RGB(255, 255, 255);
const SUBTEXT_COLOR:    Color = Color::RGB(186, 194, 222);
const PURPLE_COLOR:     Color = Color::RGB(203, 166, 247);
const PINK_COLOR:       Color = Color::RGB(243, 139, 168);
const ORANGE_COLOR:     Color = Color::RGB(250, 179, 135);
const BLACK_COLOR:      Color = Color::RGB(17,  17,  27);
const RED_COLOR:        Color = Color::RGB(255,  0,  0);




pub fn persistent_page() -> Page
{
    //===================== rects =========================
    let all_rects = vec!
    [
        //header background
        (BLACK_COLOR, (Rect::new(0, 0, 1920, 100), 0)),
    ];

    let window_center = get_center((200, 75), WINDOW_DEFAULT_SCALE);
    let padding_x = 200;
    //===================== buttons =========================
    let all_buttons = vec!
    [
        //page 1 button
        (true, PINK_COLOR, (Rect::new(window_center.pos_x - padding_x, 10, window_center.w, window_center.h), 5), 1),
        //page 2 button
        (true, PINK_COLOR, (Rect::new(window_center.pos_x + padding_x, 10, window_center.w, window_center.h), 5), 2),
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        //page_1 button text
        (17.0, (all_buttons[0].2.0.x + 9,  all_buttons[0].2.0.y + 24), "Page 1".to_string(), TEXT_COLOR),
        //page_2 button text
        (17.0, (all_buttons[1].2.0.x + 9,  all_buttons[1].2.0.y + 24), "Page 2".to_string(), TEXT_COLOR),
    ];

    //===================== images =========================
    let all_images = vec!
    [
       ((10, 10), (50, 50), format!("{}/.cache/page_system/example_1.jpg", env::home_dir().unwrap().display())),
    ];

    //===================== page creation =========================
    Page 
    {
        background_color: None,
        rects:   Some( all_rects ),
        buttons: Some( all_buttons  ),
        texts:   Some( all_text ),
        images:  Some( all_images ),
    }
}





pub fn page_1(user_input: String) -> Page
{
    let padding_y = 20;
    let red_rect_data = get_center((200, 200), WINDOW_DEFAULT_SCALE);
    let orange_rect_data = get_center((800, 200), WINDOW_DEFAULT_SCALE);
    //===================== rects =========================
    let all_rects = vec! 
    [
        // using get_center() function
        (RED_COLOR, (Rect::new(red_rect_data.pos_x, red_rect_data.pos_y + (orange_rect_data.h as i32 + padding_y), red_rect_data.w, red_rect_data.h), 100)),

        // not using it, you may preferer idk
        (ORANGE_COLOR, (Rect::new(orange_rect_data.pos_x, orange_rect_data.pos_y, orange_rect_data.w, orange_rect_data.h), 0)),
    ];

    let purple_button_data = get_center((600, 100), WINDOW_DEFAULT_SCALE);
    //===================== buttons =========================
    let all_buttons = vec!
    [
        //page 1 button
        (true, PURPLE_COLOR, (Rect::new(purple_button_data.pos_x, purple_button_data.pos_y - (orange_rect_data.h as i32 - padding_y), purple_button_data.w, purple_button_data.h), 20), 3),
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (18.0, (all_rects[1].1.0.x + 165, all_rects[1].1.0.y + 86),  "Random Orange Rectangle, Because I Can :)".to_string(), SUBTEXT_COLOR),
        //user input text
        (18.0, (all_buttons[0].2.0.x + 75, all_buttons[0].2.0.y - 25),  "Click the Button To Start Getting Input".to_string(), SUBTEXT_COLOR),
        //page 1 button text
        (25.0, (all_buttons[0].2.0.x + 15, all_buttons[0].2.0.y + 35), user_input.clone(), BLACK_COLOR),
    ];

    //===================== page creation =========================
    Page 
    {
        background_color: Some(BACKGROUND_COLOR),
        rects:   Some( all_rects ),
        buttons: Some( all_buttons  ),
        texts:   Some( all_text ),
        images:  None,
    }
}





pub fn page_2(user_input: String) -> Page
{
    let get_input_button_data = get_center((500, 100), WINDOW_DEFAULT_SCALE);
    //===================== buttons =========================
    let all_buttons = vec!
    [
        //page2 sub page button
        (true, PURPLE_COLOR,   (Rect::new(100, 150, 235, 40), 20),   4),
        (true, PURPLE_COLOR,   (Rect::new(get_input_button_data.pos_x, get_input_button_data.pos_y, get_input_button_data.w as u32, get_input_button_data.h as u32), 20), 6),
    ];


    //===================== texts =========================
    let all_text = vec!
    [
        //page 2 sub page button text
        (18.0, (all_buttons[0].2.0.x + 10, all_buttons[0].2.0.y + 7), "Go To subpage_page2".to_string(), TEXT_COLOR),
        (18.0, (all_buttons[1].2.0.x + 10, all_buttons[1].2.0.y + 7), user_input.clone(), TEXT_COLOR),
    ];

    //===================== page creation =========================
    Page 
    {
        background_color: Some(BACKGROUND_COLOR),
        rects:   None,
        buttons: Some( all_buttons  ),
        texts:   Some( all_text ),
        images:  None,
    }
}





pub fn subpage_page2() -> Page
{
    //===================== buttons =========================
    let all_buttons = vec!
    [
        //back button subpage page 2
        (true, PINK_COLOR, (Rect::new(20, 20, 50, 40), 0), 5),
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (18.0, (950, 400),  "Random Text, Because I Can :)".to_string(), SUBTEXT_COLOR),
        //back button subpage page 2 text
        (18.0, (all_buttons[0].2.0.x + 10,  all_buttons[0].2.0.y + 7),  "<-".to_string(), TEXT_COLOR),
    ];

    //===================== images =========================
    let all_images = vec!
    [
       ((500, 500), (300, 300), format!("{}/.cache/page_system/example_2.jpg", env::home_dir().unwrap().display())),
    ];

    //===================== page creation =========================
    Page 
    {
        background_color: Some(BACKGROUND_COLOR),
        rects:   None,
        buttons: Some( all_buttons ),
        texts:   Some( all_text ),
        images:  Some( all_images ),
    }
}
