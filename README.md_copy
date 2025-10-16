# basic_sdl3_rust_page_system

A small, opinionated page/state management system built with Rust and SDL3.  
This repository contains an example application and a lightweight page manager useful for games, tools, demos, or any SDL-based application that benefits from push/pop page stacks, deterministic update/rendering, and simple resource lifetimes.

This README explains the features, the dependencies you need to build and run the project, how to get started quickly, suggested workflows, and tips for troubleshooting.

---

Table of contents
- Features
- Requirements
- Platform-specific dependency hints
- Quick start (build & run)
- Example usage
- Project layout
- Development notes & recommended workflow
- Contributing
- Troubleshooting
- Roadmap
- License

---

Features
- Lightweight page system for SDL-based Rust apps
  - Push/pop pages (stack-based scene management)
  - Deterministic update loop with fixed timestep guidance
  - Centralized input/event routing to active page(s)
  - Simple render ordering (top-of-stack renders last)
- Minimal, idiomatic Rust API (designed to be easy to adapt)
- Example pages in the repository that demonstrate:
  - Menu page + game/demo page
  - Basic input handling (keyboard / mouse)
  - Simple transition pattern and resource cleanup
- Designed to work with SDL3 (native windowing, events, rendering, textures, fonts, images)

---

Requirements

1. Rust toolchain
   - rustup recommended
   - Stable Rust (we recommend Rust 1.70+ or the latest stable release)

2. SDL3 and optional SDL3 extensions (development headers / libraries)
   - SDL3 (required) — runtime and development headers
   - Optional but recommended:
     - SDL3_image (for loading PNG/JPEG etc.)
     - SDL3_ttf (for TrueType fonts)
     - SDL3_mixer (for audio, if used)
   - Note: You must install the platform-specific development packages for SDL3 and the extensions for the build to link successfully.

3. Common build tools
   - cargo (comes with rustup)
   - A C toolchain (gcc/clang / build essentials) to link native SDL libraries
   - pkg-config (to link libraries in general)
   - FontConfig (for font use)

If you need the official SDL source and instructions:
- SDL GitHub: https://github.com/libsdl-org/SDL
- SDL docs and downloads: https://www.libsdl.org/

---

Quick start

1. Clone the repository
   ```git clone https://github.com/HaruNashii/basic_sdl3_rust_page_system.git && cd basic_sdl3_rust_page_system```

2. Ensure SDL3 development libs are installed on your system (see "Platform-specific notes" above).

3. Build (debug)
   ```cargo build``` # or `cargo build --release` for release build

4. Run
   ```cargo run --release```   # or just `cargo run` for debug build

If cargo fails to link: check that your system SDL3 headers/libraries are installed and reachable (pkg-config, environment variables, or library paths).

---

Example usage (conceptual)

Below is a conceptual example of how a page system might be used. The exact API in the repository may vary slightly; use this as a guide to how the system is intended to behave.

```rust
use std::time::Duration;
use sdl3::{rect::Rect, pixels::Color};
use crate::
{
    system::
    {
        misc::center_elements::get_center,
        page_system::{Button, Page},
        input_handler::{InputEvent, InputHandler},
        state::AppState,
        window::{create_window, WINDOW_DEFAULT_SCALE},
    },
};
// Always Make "ButtonId" and "PageId" An Public Reimport In Main.rs For AppState Use
// ButtonId And PageId Need To Be Always Visible On Main.rs
//pub use your::path::if::different::file::{ButtonId, PageId};


pub mod actions;
pub mod ui;
pub mod system;


//==========================================================================================================================================================================
//====================================================================# can be a different file, like: style.rs #===========================================================
//==========================================================================================================================================================================
pub const BACKGROUND_COLOR: Color = Color::RGB(30, 30, 46);
pub const TEXT_COLOR: Color = Color::RGB(255, 255, 255);
pub const SUBTEXT_COLOR: Color = Color::RGB(186, 194, 222);
pub const PURPLE_COLOR: Color = Color::RGB(203, 166, 247);
pub const PINK_COLOR: Color = Color::RGB(243, 139, 168);
pub const ORANGE_COLOR: Color = Color::RGB(250, 179, 135);
pub const BLACK_COLOR: Color = Color::RGB(17, 17, 27);
pub const RED_COLOR: Color = Color::RGB(255, 0, 0);


//==========================================================================================================================================================================
//=======================================================================# main function recommended setup #===============================================================
//==========================================================================================================================================================================
fn main() 
{
    let (mut canvas, mut event_pump, texture_creator, ttf_context) = create_window(false);
    let input_handler = InputHandler;
    let mut app_state = AppState::new();

    //Populate Vec_Of_User_input With Page And Buttons That Receives User_Input
    app_state.push_vec_user_input(vec!
    [
        (PageId::Page1, ButtonId::ButtonPurpleInputStartPage1),
    ]);

    'running: loop 
    {
        //look the app to 60 fps (reccomended)
        std::thread::sleep(Duration::from_millis(16));
        match input_handler.poll(&mut event_pump) 
        {
            InputEvent::Click(x, y) => if let Some(button_id) = app_state.page_button_at(x, y) { button_action(&mut app_state, button_id); },
            InputEvent::Text(s) => app_state.handle_text(s),
            InputEvent::Backspace => app_state.handle_backspace(),
            InputEvent::Submit => app_state.submit_input(),
            InputEvent::Quit => break 'running,
            InputEvent::None => {}
        }
        app_state.render(&mut canvas, &texture_creator, &ttf_context);
    }
}


//==========================================================================================================================================================================
//===============================================================# can be a different file, like: buttons_actions.rs #======================================================
//==========================================================================================================================================================================
// Define Buttons Actions
pub fn button_action(app_state: &mut AppState, button_id: ButtonId) 
{
    if !app_state.capturing_input.0
    {
        match button_id 
        {
            ButtonId::ButtonPage1 =>   app_state.current_page = PageId::Page1,
            ButtonId::ButtonSubPage => app_state.current_page = PageId::Page1SubPage,
            ButtonId::ButtonBack =>    app_state.current_page = PageId::Page1,
            // Non Handle Buttons Will Be Considered User Input Buttons
            _=> app_state.capturing_input = (true, Some(button_id)),
        }
    }
}


//==========================================================================================================================================================================
//===============================================================# can be a different file, like: pages.rs #================================================================
//==========================================================================================================================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Defines The ID for your Pages
pub enum PageId 
{
    Persistent,
    Page1,
    Page1SubPage,
}
#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(usize)]
/// Defines The ID for your Buttons
pub enum ButtonId 
{
    ButtonPage1,
    ButtonPurpleInputStartPage1,
    ButtonSubPage,
    ButtonBack,
}


impl Page 
{
    /// Link PageId To The Page, Make the AppState Be Able To Create The Page Based On The Assigned
    /// PageId
    pub fn create_from_id(id: PageId, option_user_input: Option<Vec<String>>) -> Self 
    {
        match id 
        {
            PageId::Persistent => Self::persistent_page(),
            PageId::Page1 => Self::page_1(option_user_input.expect("Page1 Received User Input That Doesn't Exist, Did you Set This Page To Receive Input?")),
            PageId::Page1SubPage => Self::subpage_page1(),
        }
    }


    // Define Your Pages Here:
    pub fn persistent_page() -> Self 
    {
        //===================== variables =========================
        let window_center = get_center((200, 75), WINDOW_DEFAULT_SCALE);

        //===================== rects =========================
        let all_rects = vec!
        [
            (BLACK_COLOR, (Rect::new(0, 0, WINDOW_DEFAULT_SCALE.0, 100), 0))
        ];
    
        //===================== buttons =========================
        let all_buttons = vec!
        [
            Button { enabled: true, color: PINK_COLOR, rect: Rect::new(window_center.pos_x, 10, window_center.w, window_center.h), radius: 5, id: ButtonId::ButtonPage1 },
        ];

        //===================== texts =========================
        let all_text = vec!
        [
            //page_1 button text
            (17.0, (all_buttons[0].rect.x + 9, all_buttons[0].rect.y + 24), "Page 1".to_string(), TEXT_COLOR),
        ];

        //===================== images =========================
        let all_images = vec!
        [
            ((10, 10), (50, 50), "path_to_your_image".to_string())
        ];

        //===================== page creation =========================
        Self { has_persistant_page: false, id: PageId::Persistent, background_color: None, rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_text), images: Some(all_images) }
    }

    pub fn page_1(user_input: Vec<String>) -> Self 
    {
        //===================== variables =========================
        let purple_button_data = get_center((600, 100), WINDOW_DEFAULT_SCALE);
        let subpage_button_data = get_center((235, 40), WINDOW_DEFAULT_SCALE);

        //===================== buttons =========================
        let all_buttons = vec!
        [
            Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(subpage_button_data.pos_x, 150, subpage_button_data.w, subpage_button_data.h), radius: 20, id: ButtonId::ButtonSubPage },
            Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(purple_button_data.pos_x, purple_button_data.pos_y, purple_button_data.w, purple_button_data.h), radius: 5, id: ButtonId::ButtonPurpleInputStartPage1 },
        ];

        //===================== texts =========================
        let all_text = vec!
        [
            (18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "Go To subpage_page1".to_string(), TEXT_COLOR),
            (18.0, (all_buttons[1].rect.x + 75, all_buttons[1].rect.y - 25), "Click the Button To Start Getting Input".to_string(), SUBTEXT_COLOR),
            (25.0, (all_buttons[1].rect.x + 15, all_buttons[1].rect.y + 35), user_input[0].clone(), BLACK_COLOR),
        ];

        //===================== page creation =========================
        Self { has_persistant_page: true, id: PageId::Page1, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }
    }

    pub fn subpage_page1() -> Self 
    {
        //===================== buttons =========================
        let all_buttons = vec!
        [
            Button { enabled: true, color: PINK_COLOR, rect: Rect::new(20, 20, 50, 40), radius: 0, id: ButtonId::ButtonBack }
        ];

        //===================== texts =========================
        let all_text = vec!
        [
            (18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "<-".to_string(), TEXT_COLOR)
        ];

        //===================== page creation =========================
        Self { has_persistant_page: false, id: PageId::Page1SubPage, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }
    }
}
```

This pattern lets you:
- Swap entire screens/pages cleanly
- Keep page-specific state encapsulated
- Easily implement pause screens, modal dialogs (by stacking pages)

---

Project layout (high-level)
- Cargo.toml — Rust project configuration
- assets/ — images, fonts, audio used by examples (if included)
- src/
  - main.rs — application entrypoint
  - ui/your_pages.rs — page implementations (menu/game/demo)
  - ui/style.rs — style of page elements (buttons/rects/texts)
  - actions/buttons_actions.rs - actions that buttons take when pressed

---

Development notes & recommended workflow
- Use clippy
  - cargo clippy
- If you add native SDL dependencies, version them in the README and consider documenting how to pin alternate SDL installations via PKG_CONFIG_PATH or PKG_CONFIG_SYSROOT_DIR.
- When adding features (audio, fonts, etc.), gate them behind Cargo features and document required system dependencies.

Suggested Cargo features (example)
- features:
  - "image" => enables SDL_image usage
  - "ttf" => enables SDL_ttf usage
  - "audio" => enables SDL_mixer usage

---

Contributing
- Fork the repository, create a feature branch, and open a pull request.
- When opening PRs:
  - Include a short description of the change
  - Add or update examples demonstrating the new/changed behavior
  - Don't Run cargo fmt and run cargo clippy
- For larger proposals, open an issue first to discuss the design.

---

Troubleshooting

Linker errors (cannot find -lSDL3, undefined references)
- Ensure the SDL3 dev libraries are installed and visible to your linker.
- On Linux, verify pkg-config can find SDL3: pkg-config --cflags --libs sdl3
- Set PKG_CONFIG_PATH to the directory where the SDL3 .pc files are installed.

Missing headers at compile time
- Install the development package (headers). On Debian/Ubuntu that is normally libsdl3-dev (if available) or build SDL3 from source.

Runtime errors on Windows
- Make sure SDL3 DLLs are either on the PATH or next to the executable.
- Ensure you built/run with the same runtime (MSVC vs MinGW) as your SDL binaries.

---

Roadmap / Ideas
- Add examples showing:
  - Smooth transitions (fade/slide) between pages
  - Resource manager for textures/fonts/sounds
  - Input remapping and configurable controls
- Add unit/integration tests for the page manager logic

---

License
This Project are licensed under the MIT licence. Please see the [license](https://github.com/HaruNashii/basic_sdl3_rust_page_system/blob/main/LICENSE) file for more information. tl;dr you can do whatever you want as long as you include the original copyright and license notice in any copy of the software/source.

---

Acknowledgements & References
- SDL: https://www.libsdl.org/
- SDL GitHub: https://github.com/libsdl-org/SDL
- Rust: https://www.rust-lang.org/

---
