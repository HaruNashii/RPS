use sdl3::
{
    rect::Rect,
    pixels::Color,
    render::Texture,
};





pub struct Page<'a>
{
    pub background_color: Option<Color>,
    pub rects:   Option<Vec< (Color, Rect) >>,
    pub buttons: Option<Vec< (bool, Color, Rect, u16) >>,
    pub texts:   Option<Vec< (f64, (i32, i32), String, Color) >>,
    pub images:  Option<Vec< (Texture<'a>, Rect) >>,
}





pub const COLOR_CHANGE_WHEN_SELECTED: (u8, u8, u8) = (25, 25, 25);
const BACKGROUND_COLOR: Color = Color::RGB(30,  30,  46);
const TEXT_COLOR:       Color = Color::RGB(255, 255, 255);
const SUBTEXT_COLOR:    Color = Color::RGB(186, 194, 222);
const PURPLE_COLOR:     Color = Color::RGB(203, 166, 247);
const PINK_COLOR:       Color = Color::RGB(243, 139, 168);
const ORANGE_COLOR:     Color = Color::RGB(250, 179, 135);
const BLACK_COLOR:      Color = Color::RGB(17,  17,  27);





pub fn persistent_page() -> Page<'static>
{
    //===================== rects =========================
    let all_rects = vec!
    [
        //header background
        (BLACK_COLOR, Rect::new(0, 0, 1920, 100)),
    ];

    //===================== buttons =========================
    let all_buttons = vec!
    [
        //page 1 button
        (true, PINK_COLOR, Rect::new(550, 10, 200, 75), 1),
        //page 2 button
        (true, PINK_COLOR, Rect::new(850, 10, 200, 75), 2),
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        //page_1 button text
        (17.0, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 24), "Page 1".to_string(), TEXT_COLOR),
        //page_2 button text
        (17.0, (all_buttons[1].2.x + 9, all_buttons[1].2.y + 24), "Page 2".to_string(), TEXT_COLOR),
    ];

    //===================== page creation =========================
    Page 
    {
        background_color: None,
        rects:   Some( all_rects ),
        buttons: Some( all_buttons  ),
        texts:   Some( all_text ),
        images:  None,
    }
}





pub fn page_1() -> Page<'static>
{
    //===================== rects =========================
    let all_rects = vec! 
    [
        //random orange thing, just because i can :)
        (ORANGE_COLOR, Rect::new(900, 600, 200, 200)),
    ];

    //===================== buttons =========================
    let all_buttons: Vec<(bool, Color, Rect, u16)> = vec!
    [
        //page 1 button
        (true, PURPLE_COLOR, Rect::new(200, 105, 940, 40), 3),
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        //page 1 button text
        (25.0, (all_buttons[0].2.x + 70, all_buttons[0].2.y + 5), "page 1 button, yay".to_string(), BLACK_COLOR),
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





pub fn page_2() -> Page<'static>
{
    //===================== buttons =========================
    let all_buttons = vec!
    [
        //page2 sub page button
        (true, PURPLE_COLOR,   Rect::new(10, 105, 940, 40), 4),
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        //page 2 sub page button text
        (18.0, (all_buttons[0].2.x + 10, all_buttons[0].2.y + 7), "Go To subpage_page2".to_string(), TEXT_COLOR),
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





pub fn subpage_page2() -> Page<'static>
{
    //===================== buttons =========================
    let all_buttons = vec!
    [
        //back button subpage page 2
        (true, PINK_COLOR, Rect::new(20, 20, 50, 40), 5),
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (18.0, (950, 400),  "Random Text, Because I Can :)".to_string(), SUBTEXT_COLOR),
        //back button subpage page 2 text
        (18.0, (all_buttons[0].2.x + 10,  all_buttons[0].2.y + 7),  "<-".to_string(), TEXT_COLOR),
    ];

    //===================== page creation =========================
    Page 
    {
        background_color: Some(BACKGROUND_COLOR),
        rects:   None,
        buttons: Some( all_buttons ),
        texts:   Some( all_text ),
        images:  None,
    }
}
