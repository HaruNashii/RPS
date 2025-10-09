use std::time::Duration;

use crate::
{
    buttons::button_action,
    input_handler::{MouseInput, KeyboardInput},
    pages::{persistent_page, page_1, page_2, subpage_page2},
    window::{create_window, render_page}
};





pub mod pages;
pub mod window;
pub mod input_handler;
pub mod sdl3_generators;
pub mod buttons;





fn main()
{
    let (mut canvas, mut event_pump, texture_creator) = create_window();
    let mut page_to_render: u8 = 1;
    let mut user_input = [String::new(), String::new()];
    let mut get_user_input = (false, 0);

    loop
    {
        std::thread::sleep(Duration::from_millis(32));
        let mut persistent_page = persistent_page();
        let window_scale: (u32, u32) = (canvas.window().size().0, canvas.window().size().1);
        if get_user_input.0 { (user_input[get_user_input.1], get_user_input.0) = (&mut event_pump, user_input[get_user_input.1].to_string(), get_user_input.0).handle_keyboard_input(); }



        match page_to_render
        {
            1 => 
            {
                let mut page_1 = page_1(user_input[0].clone());


                let (button_clicked, new_buttons) = ((Some(&persistent_page.buttons.clone().unwrap()), &page_1.buttons.clone().unwrap()), &mut event_pump, window_scale).handle_mouse_input();
                persistent_page.buttons = new_buttons.0;
                page_1.buttons = Some(new_buttons.1);

                button_action(button_clicked, &mut get_user_input, &mut page_to_render);
                render_page(page_1, Some(persistent_page), &mut canvas, &texture_creator);
            },

            2 =>
            {
                let mut page_2 = page_2(user_input[1].clone());

                let (button_clicked, new_buttons) = ((Some(&persistent_page.buttons.clone().unwrap()), &page_2.buttons.clone().unwrap()), &mut event_pump, window_scale).handle_mouse_input();
                persistent_page.buttons = new_buttons.0;
                page_2.buttons = Some(new_buttons.1);

                button_action(button_clicked, &mut get_user_input, &mut page_to_render);
                render_page(page_2, Some(persistent_page), &mut canvas, &texture_creator);
            },

            3 =>
            {
                let mut subpage_page2 = subpage_page2();

                let (button_clicked, new_buttons) = ((None, &subpage_page2.buttons.clone().unwrap()), &mut event_pump, window_scale).handle_mouse_input();
                subpage_page2.buttons = Some(new_buttons.1);

                button_action(button_clicked, &mut (false, 0), &mut page_to_render);
                render_page(subpage_page2, None, &mut canvas, &texture_creator);
            },

            _=>{},
        }
    }
}
