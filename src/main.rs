use std::time::Duration;

use crate::
{
    buttons::{button_action, PAGE_TO_RENDER},
    input_handler::MouseInput,
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

    loop
    {
        std::thread::sleep(Duration::from_millis(32));

        let persistent_page = persistent_page();
        let window_scale: (u32, u32) = (canvas.window().size().0, canvas.window().size().1);

        match unsafe{PAGE_TO_RENDER}
        {
            1 => 
            {
                let page_1 = page_1();
                let (_, button_clicked) = ((Some(&persistent_page.buttons.clone().unwrap()), &page_1.buttons.clone().unwrap()), &mut event_pump, window_scale).handle_mouse_input();
                button_action(button_clicked);
                render_page(page_1, Some(persistent_page), &mut canvas, &texture_creator);
            },

            2 =>
            {
                let page_2 = page_2();
                let (_, button_clicked) = ((Some(&persistent_page.buttons.clone().unwrap()), &page_2.buttons.clone().unwrap()), &mut event_pump, window_scale).handle_mouse_input();
                button_action(button_clicked);
                render_page(page_2, Some(persistent_page), &mut canvas, &texture_creator);
            },

            3 =>
            {
                let subpage_page2 = subpage_page2();
                let (_, button_clicked) = ((None, &subpage_page2.buttons.clone().unwrap()), &mut event_pump, window_scale).handle_mouse_input();
                button_action(button_clicked);
                render_page(subpage_page2, None, &mut canvas, &texture_creator);
            },

            _=>{},
        }
    }
}
