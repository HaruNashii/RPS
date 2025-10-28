# RPS - Rust Page System

A small, opinionated page/state management system built with Rust and SDL3.  
This repository contains an example application and a lightweight page manager useful for games, tools, demos, or any SDL-based application that benefits from push/pop page stacks, deterministic update/rendering, and simple resource lifetimes.

This README explains the features, the dependencies you need to build and run the project, how to get started quickly, suggested workflows, and tips for troubleshooting.

---

## Table of contents
- [Features](#features)
- [Requirements](#requirements)
- [Example usage](#example-usage)
- [Project layout](#suggested-project-layout)
- [Development notes & recommended workflow](#development-notes--recommended-workflow)
- [Contributing](#contributing)
- [Troubleshooting](#troubleshooting)
- [Roadmap](#roadmap--ideas)
- [License](#license)
- [Acknowledgements & References](#acknowledgements--references)
  
---

## Features
- Lightweight page system for SDL-based Rust apps
  - Push/pop pages (stack-based scene management)
  - Centralized input/event routing to active page(s)
  - Simple render ordering (top-of-stack renders last)
- Minimal, idiomatic Rust API (designed to be easy to adapt)
- Example pages in the repository that demonstrate:
  - Simple single Page with Persistent Elements and UserInputText
  - Basic input handling (keyboard / mouse)
  - More complex example with Multiples Pages, Persistent Elements and UserInputText
- Designed to work with SDL3 (native windowing, events, rendering, textures, fonts, images)

---

## Requirements

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
- SDL3 Rust-Bindings Github: https://github.com/vhspace/sdl3-rs

---

## Example usage
Below is a example of how a page system might be used. The exact API in the repository may vary slightly; use this as a guide to how the system is intended to behave.
for more accurate examples you can see them [here](https://github.com/HaruNashii/RPS/tree/bdadc5c7d4d283b3438400ebcb894370032b1765/examples)

<details> <summary>Simple Conceptual Example</summary>

```rust
use std::{env, time::Duration};
use sdl3::{pixels::Color, rect::Rect, sys::render::SDL_LOGICAL_PRESENTATION_STRETCH};
use rust_page_system::
{
    Button, 
    Renderer
    misc::{center_elements::get_center, vec::GetOrCreate}, 
    system::{input_handler::InputHandler, page_system::{Page, PageData, PersistentElements}, renderer::RendererConfig, state::AppState, window::{create_window, get_monitor_refresh_rate, WindowConfig}}, 
};



//==========================================================================================================================================================================
//=======================================================================# main function recommended setup #===============================================================
//==========================================================================================================================================================================
fn main() 
{
    let window_config = WindowConfig
    {
        window_title: "SimpleExample".to_string(),
        icon: (false, None),
        // Recommended to start with 16:9 aspect ratio
        start_window_size: (800, 450),
        // Recommended to have minimum size with 16:9 aspect ratio
        window_minimum_size: (800, 450),
        resizable: true,
        centered: true,
        // By Default SDL_LOGICAL_PRESENTATION_STRETCH Is Set, Only Setting It Here For Demonstration Purpose 
        different_sdl_presentation_mode: Some(SDL_LOGICAL_PRESENTATION_STRETCH), 
        font: ("JetBrainsMono".to_string(), Some("Bold".to_string()))
    };
    let mut window_modules = create_window(window_config);

    //bool is reffered to the rollback pages system, with "Mouse side buttons" or ("Alt" + "Arrows Keys") | (false = Page Rollback On), (true = Page Rollback Off)
    let mut input_handler = InputHandler::new(false);
    let mut app_state = AppState::new(PageId::Page1, window_modules.canvas.window().size());
    let mut page_data = PageData::new(&app_state);

    let renderer_config = RendererConfig
    {
        canvas: window_modules.canvas, 
        texture_creator: &window_modules.texture_creator, 
        ttf_context: &window_modules.ttf_context,
        font_path: &window_modules.font_path,
        decrease_color_when_selected: Some((25, 25, 25)),
        selection_color: Some((0, 0, 200, 125)),

    };
    let mut renderer = Renderer::new(renderer_config);

    populate_page_data(&mut page_data);

    loop 
    {
        //using 900 / your_refresh_rate to a very crispy experience
        std::thread::sleep(Duration::from_millis(900 / get_monitor_refresh_rate()));
        app_state.update_window_size(renderer.canvas.window().size().0, renderer.canvas.window().size().1);
        input_handler.handle_input(&mut window_modules.event_pump, &mut window_modules.clipboard_system, &mut page_data, &mut app_state, button_action);
        page_data.create_current_page(&mut app_state);
        renderer.render(&page_data, &mut app_state, &input_handler);
    }
}



//==========================================================================================================================================================================
//===============================================================# can be a different file, like: buttons_actions.rs #======================================================
//==========================================================================================================================================================================
pub fn button_action(app_state: &mut AppState<PageId, ButtonId>, button_id: &ButtonId, app_data: &mut PageData<PageId, ButtonId>) 
{
    if !app_state.capturing_input.0
    {
        if &ButtonId::ButtonPage1    == button_id {app_state.change_current_page(app_data, PageId::Page1); return};
        if &ButtonId::ButtonSubPage  == button_id {app_state.change_current_page(app_data, PageId::Page1SubPage); return};
        if &ButtonId::ButtonBack     == button_id {app_state.change_current_page(app_data, PageId::Page1); return};
        // Non Handle Buttons Will Be Considered User Input Buttons
        app_state.capturing_input = (true, Some(*button_id));
    }
}



//==========================================================================================================================================================================
//===============================================================# can be a different file, like: setup_page_data.rs #======================================================
//==========================================================================================================================================================================
pub fn populate_page_data(page_data: &mut PageData<PageId, ButtonId>)
{
    page_data.push_page_link
    (
        Some(vec![(PageId::Page1SubPage, subpage_page1)]),
        Some(vec![(PageId::Page1, page_1)])
    );
}



//==========================================================================================================================================================================
//====================================================================# can be a different file, like: style.rs (or not even exist) #=======================================
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
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[repr(usize)]
/// Defines The ID for your Buttons
pub enum ButtonId 
{
    ButtonPage1,
    ButtonPurpleInputStartPage1,
    ButtonSubPage,
    ButtonBack,
}

// Define Your Pages Here:
pub fn persistent_elements() -> PersistentElements<PageId, ButtonId>
{
    //===================== rects =========================
    let all_rects = vec! [ (BLACK_COLOR, (Rect::new(0, 0, 1920, 100), 0)) ];

    //===================== texts =========================
    let all_text = vec! [ (17.0, (825, 34), "This Is A Persistent Element".to_string(), TEXT_COLOR), ];

    //===================== images =========================
    let all_images = vec!
    [
        ((10, 10), (50, 50), format!("{}/.cache/page_system/example_1.jpg", env::home_dir().unwrap().display()))
    ];

    //===================== page creation =========================
    PersistentElements { id: PageId::Persistent, background_color: None, rects: Some(all_rects), buttons: None, texts: Some(all_text), images: Some(all_images) }
}

pub fn page_1(user_input: &mut Vec<String>) -> Page<PageId, ButtonId>
{
    //===================== variables =========================
    let purple_button_data = get_center((600, 100), (1920, 1080));
    let subpage_button_data = get_center((235, 40), (1920, 1080));

    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(subpage_button_data.pos_x, 150, subpage_button_data.w, subpage_button_data.h), radius: 20, id: ButtonId::ButtonSubPage},
        Button { enabled: true, color: PURPLE_COLOR, rect: Rect::new(purple_button_data.pos_x, purple_button_data.pos_y, purple_button_data.w, purple_button_data.h), radius: 5, id: ButtonId::ButtonPurpleInputStartPage1},
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "Go To subpage_page1".to_string(), TEXT_COLOR),
        (18.0, (all_buttons[1].rect.x + 75, all_buttons[1].rect.y - 25), "Click the Button To Start Getting Input".to_string(), SUBTEXT_COLOR),
        (25.0, (all_buttons[1].rect.x + 15, all_buttons[1].rect.y + 35), user_input.get_or_create(0), BLACK_COLOR),
    ];

    //===================== page creation =========================
    Page { has_userinput: Some(vec![(PageId::Page1, ButtonId::ButtonPurpleInputStartPage1)]), has_persistent_elements: Some(vec![(PageId::Persistent, persistent_elements)]), has_transition: None, id: PageId::Page1, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }

}

pub fn subpage_page1() -> Page<PageId,ButtonId>
{
    //===================== buttons =========================
    let all_buttons = vec! [ Button { enabled: true, color: PINK_COLOR, rect: Rect::new(20, 20, 50, 40), radius: 0, id: ButtonId::ButtonBack}];

    //===================== texts =========================
    let all_text = vec! [ (18.0, (all_buttons[0].rect.x + 10, all_buttons[0].rect.y + 7), "<-".to_string(), TEXT_COLOR) ];

    //===================== page creation =========================
    Page { has_userinput: None, has_persistent_elements: None, has_transition: None, id: PageId::Page1SubPage, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: None }
}
```

</details>

This pattern lets you:
- Swap entire screens/pages cleanly
- Keep page-specific state encapsulated
- Easily implement pause screens, modal dialogs (by stacking pages)

---

## Suggested Project layout
- Cargo.toml — Rust project configuration
- assets/ — images, fonts, audio used by examples (if included)
- src/
  - main.rs — application entrypoint
  - ui/your_pages.rs — page implementations (menu/game/demo)
  - ui/style.rs — style of page elements (buttons/rects/texts)
  - actions/buttons_actions.rs - actions that buttons take when pressed

---

## Development notes & recommended workflow
- Use cargo clippy
- When adding features (audio, fonts, etc.), gate them behind Cargo features and document required system dependencies.

Suggested Cargo features (example)
- features:
  - "image" => enables SDL_image usage
  - "ttf" => enables SDL_ttf usage
  - "audio" => enables SDL_mixer usage

---

## Contributing
- Fork the repository, create a feature branch, and open a pull request.
- When opening PRs:
  - Include a short description of the change
  - Always run cargo clippy & cargo fmt (nightly)
- For larger proposals, open an issue first to discuss the design.

---

## Troubleshooting
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

## Roadmap / Ideas
- Deterministic update loop with fixed timestep guidance
- Resource manager for textures/fonts/sounds
- Input remapping and configurable controls
- Add more unit/integration tests for the page manager logic

---

## License
This Project are licensed under the MIT licence. Please see the [license](https://github.com/HaruNashii/RPS/blob/main/LICENSE) file for more information. tl;dr you can do whatever you want. :)

---

## Acknowledgements & References
- SDL: https://www.libsdl.org/
- SDL GitHub: https://github.com/libsdl-org/SDL
- SDL3 Rust-Bindings: https://github.com/vhspace/sdl3-rs
- Rust: https://www.rust-lang.org/

---
