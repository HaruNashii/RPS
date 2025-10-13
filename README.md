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
   cargo run --release   # or just `cargo run` for debug build

If cargo fails to link: check that your system SDL3 headers/libraries are installed and reachable (pkg-config, environment variables, or library paths).

---

Example usage (conceptual)

Below is a conceptual example of how a page system might be used. The exact API in the repository may vary slightly; use this as a guide to how the system is intended to behave.

```rust
Yet Todo!
```

This pattern lets you:
- Swap entire screens/pages cleanly
- Keep page-specific state encapsulated
- Easily implement pause screens, modal dialogs, transitions (by stacking pages)

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
This Project are licensed under the MIT licence. Please see the licence file for more information. tl;dr you can do whatever you want as long as you include the original copyright and license notice in any copy of the software/source.

---

Acknowledgements & References
- SDL: https://www.libsdl.org/
- SDL GitHub: https://github.com/libsdl-org/SDL
- Rust: https://www.rust-lang.org/

---
