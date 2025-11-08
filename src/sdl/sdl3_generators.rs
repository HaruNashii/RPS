use gif::{ColorOutput, DecodeOptions};
use gif_dispose::Screen;
use include_dir::Dir;
use once_cell::sync::Lazy;
use sdl3::{
    image::LoadTexture,
    iostream::IOStream,
    pixels::{Color, PixelFormat},
    rect::Rect,
    render::{Texture, TextureCreator},
    surface::Surface,
    ttf::Sdl3TtfContext,
    video::WindowContext
};
use std::{
    collections::HashMap,
    io::Cursor,
    path::{Path, PathBuf},
    sync::Mutex,
    time::Instant
};

// A struct to hold all frames, delays and timing info for one GIF
struct GifFrames
{
    frames: Vec<Vec<u8>>, // RGBA pixel data for each frame
    delays: Vec<u32>,     // delay per frame in milliseconds
    width: u32,
    height: u32,
    total_duration: u32,
    start_time: Instant // when the animation started playing
}

// A global cache keyed by file path, protected by a Mutex
static GIF_CACHE: Lazy<Mutex<HashMap<String, GifFrames>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn load_gif_texture_from_bytes<'a>(bytes: &[u8], texture_creator: &'a TextureCreator<WindowContext>, cache_key: &str) -> Option<Texture<'a>>
{
    // 🔒 Reuse the same global cache
    let mut cache = GIF_CACHE.lock().expect("GIF cache poisoned");

    let entry = cache.entry(cache_key.to_string()).or_insert_with(|| {
        let cursor = Cursor::new(bytes);
        let mut opts = DecodeOptions::new();
        opts.set_color_output(ColorOutput::Indexed);
        let mut decoder = opts.read_info(cursor).unwrap_or_else(|_| panic!("Failed to read embedded GIF info for '{}'", cache_key));
        let mut screen = Screen::new_decoder(&decoder);

        let width = decoder.width() as u32;
        let height = decoder.height() as u32;
        let mut frames = Vec::new();
        let mut delays = Vec::new();

        while let Some(frame) = decoder.read_next_frame().expect("Error decoding embedded GIF frame")
        {
            screen.blit_frame(frame).expect("Error composing embedded GIF frame");

            let (cow_pixels, _, _) = screen.pixels_rgba().to_contiguous_buf();
            let mut rgba_bytes = Vec::with_capacity(cow_pixels.len() * 4);
            for px in cow_pixels.iter()
            {
                rgba_bytes.extend_from_slice(&[px.r, px.g, px.b, px.a]);
            }
            frames.push(rgba_bytes);

            let mut d = (frame.delay as u32) * 10;
            if d == 0
            {
                d = 100;
            }
            delays.push(d);
        }

        let total = delays.iter().sum::<u32>().max(1);
        GifFrames { frames, delays, width, height, total_duration: total, start_time: Instant::now() }
    });

    // 🕒 Pick frame based on elapsed time
    let elapsed_ms = entry.start_time.elapsed().as_millis() as u32;
    let current = elapsed_ms % entry.total_duration;
    let mut acc = 0;
    let mut idx = 0;
    for (i, delay) in entry.delays.iter().enumerate()
    {
        if current < acc + *delay
        {
            idx = i;
            break;
        }
        acc += *delay;
    }

    // 🖼️ Convert current frame to texture
    let mut pixels = entry.frames[idx].clone();
    let surface = Surface::from_data(pixels.as_mut_slice(), entry.width, entry.height, entry.width * 4, PixelFormat::ABGR8888).ok()?;
    texture_creator.create_texture_from_surface(&surface).ok()
}

fn load_gif_texture<'a>(path: &str, texture_creator: &'a TextureCreator<WindowContext>) -> Option<Texture<'a>>
{
    let mut cache = GIF_CACHE.lock().expect("GIF cache poisoned");
    let entry = cache.entry(path.to_owned()).or_insert_with(|| {
        let mut opts = DecodeOptions::new();
        opts.set_color_output(ColorOutput::Indexed);

        let file = std::fs::File::open(path).unwrap_or_else(|_| panic!("Failed to open GIF {}", path));
        let mut decoder = opts.read_info(file).unwrap_or_else(|_| panic!("Failed to read GIF info for {}", path));

        // Use gif_dispose to compose partial frames
        let mut screen = Screen::new_decoder(&decoder);

        let width = decoder.width() as u32;
        let height = decoder.height() as u32;
        let mut frames: Vec<Vec<u8>> = Vec::new();
        let mut delays: Vec<u32> = Vec::new();

        while let Some(frame) = decoder.read_next_frame().expect("Error decoding GIF frame")
        {
            screen.blit_frame(frame).expect("Error composing GIF frame");

            // `to_contiguous_buf()` returns (pixels, width, height)
            let (cow_pixels, _w, _h) = screen.pixels_rgba().to_contiguous_buf();

            // Flatten Rgba<u8> pixels into Vec<u8>
            let mut rgba_bytes = Vec::with_capacity(cow_pixels.len() * 4);
            for px in cow_pixels.iter()
            {
                rgba_bytes.push(px.r);
                rgba_bytes.push(px.g);
                rgba_bytes.push(px.b);
                rgba_bytes.push(px.a);
            }

            frames.push(rgba_bytes);

            let mut d = (frame.delay as u32) * 10;
            if d == 0
            {
                d = 100;
            }
            delays.push(d);
        }


        let total = delays.iter().sum::<u32>().max(1);
        GifFrames { frames, delays, width, height, total_duration: total, start_time: Instant::now() }
    });


    // choose frame based on elapsed time
    let elapsed_ms = entry.start_time.elapsed().as_millis() as u32;
    let current = elapsed_ms % entry.total_duration;
    let mut acc = 0;
    let mut idx = 0;
    for (i, delay) in entry.delays.iter().enumerate()
    {
        if current < acc + *delay
        {
            idx = i;
            break;
        }
        acc += *delay;
    }


    let mut pixels = entry.frames[idx].clone();
    let surface = Surface::from_data(pixels.as_mut_slice(), entry.width, entry.height, entry.width * 4, PixelFormat::ABGR8888).ok()?;
    texture_creator.create_texture_from_surface(&surface).ok()
}

pub trait GenerateText
{
    fn generate_text(&mut self, font_path: &str) -> Vec<(Texture<'_>, Rect)>;
}
impl GenerateText for (&mut Vec<(f64, (i32, i32), String, Color)>, &TextureCreator<WindowContext>, &Sdl3TtfContext)
{
    ///Helper Function That Generates The Page Texts
    fn generate_text(&mut self, font_path: &str) -> Vec<(Texture<'_>, Rect)>
    {
        let mut vector_to_send = Vec::new();
        for font_content in &mut *self.0
        {
            let text_content = if font_content.2.is_empty() { " " } else { &font_content.2 };
            let font = self.2.load_font(font_path, font_content.0 as f32).expect("Failed to load font");
            let lines: Vec<&str> = text_content.split('\n').collect();
            let mut current_y = font_content.1.1;

            for line in lines
            {
                let render_text = if line.is_empty() { " " } else { line };
                let surface = font.render(render_text).blended(font_content.3).expect("Failed to blend font");
                let surface_argb = surface.convert_format(sdl3::pixels::PixelFormat::ARGB8888).expect("Failed to convert surface to ARGB8888");
                let texture = self.1.create_texture_from_surface(&surface_argb).expect("Failed to create ARGB font texture");
                let rect = Rect::new(font_content.1.0, current_y, surface.width(), surface.height());
                vector_to_send.push((texture, rect));
                current_y += surface.height() as i32;
            }

            drop(font);
        }
        vector_to_send
    }
}


// === Embed your assets folder ===
pub trait GenerateImage
{
    fn generate_image(&mut self, option_assets: Option<&Dir>) -> Vec<(Texture<'_>, Rect)>;
}
impl GenerateImage for (&mut Vec<((i32, i32), (u32, u32), String)>, &TextureCreator<WindowContext>)
{
    ///All files inside the root/assets will be embedded, but for now sdl3-rust just support
    ///rendering per BMP files, so only bmp files will be possible to use from embedded
    fn generate_image(&mut self, option_assets: Option<&Dir>) -> Vec<(Texture<'_>, Rect)>
    {
        let mut textures = Vec::new();

        for (pos, size, path_str) in &mut *self.0
        {
            if let Some(assets) = option_assets
            {
                // Normalize path (cross-platform)
                let normalized = {
                    let path = PathBuf::from(&*path_str);
                    path.components().filter_map(|c| c.as_os_str().to_str()).collect::<Vec<_>>().join("/")
                };

                // === 1️⃣ Try exact embedded path match ===
                if let Some(file) = assets.get_file(&normalized)
                {
                    let data = file.contents();
                    let lower = normalized.to_lowercase();
                    if lower.ends_with(".gif")
                    {
                        let cache_key = format!("embedded:{}", normalized);
                        if let Some(tex) = load_gif_texture_from_bytes(data, self.1, &cache_key)
                        {
                            let rect = Rect::new(pos.0, pos.1, size.0, size.1);
                            textures.push((tex, rect));
                            continue;
                        }
                    }
                    if lower.ends_with(".bmp")
                    {
                        let mut stream = IOStream::from_bytes(file.contents()).expect("Failed to create IOStream from embedded data");
                        let surface = Surface::load_bmp_rw(&mut stream).unwrap_or_else(|_| panic!("Failed to load embedded BMP '{}'", normalized));
                        let texture = self.1.create_texture_from_surface(&surface).unwrap_or_else(|_| panic!("Failed to create texture for '{}'", normalized));
                        let rect = Rect::new(pos.0, pos.1, size.0, size.1);
                        textures.push((texture, rect));
                        continue;
                    };
                }

                // === 2️⃣ Recursive lookup by filename only ===
                if let Some(file) = assets.files().find(|f| f.path().file_name() == Path::new(&normalized).file_name())
                {
                    let data = file.contents();
                    let lower = normalized.to_lowercase();
                    if lower.ends_with(".gif")
                    {
                        let cache_key = format!("embedded:{}", normalized);
                        if let Some(tex) = load_gif_texture_from_bytes(data, self.1, &cache_key)
                        {
                            let rect = Rect::new(pos.0, pos.1, size.0, size.1);
                            textures.push((tex, rect));
                            continue;
                        }
                    }
                    if lower.ends_with(".bmp")
                    {
                        let mut stream = IOStream::from_bytes(file.contents()).expect("Failed to create IOStream from embedded data (search mode)");
                        let surface = Surface::load_bmp_rw(&mut stream).unwrap_or_else(|_| panic!("Failed to load embedded BMP '{}'", normalized));
                        let texture = self.1.create_texture_from_surface(&surface).unwrap_or_else(|_| panic!("Failed to create texture for '{}'", normalized));
                        let rect = Rect::new(pos.0, pos.1, size.0, size.1);
                        textures.push((texture, rect));
                        continue;
                    };
                }
            }

            // === 3️⃣ Fallback: load from disk ===
            let lower = path_str.to_lowercase();
            if Path::new(path_str).exists()
            {
                if lower.ends_with(".gif")
                    && let Some(tex) = load_gif_texture(path_str, self.1)
                {
                    textures.push((tex, Rect::new(pos.0, pos.1, size.0, size.1)));
                    continue;
                }
                if lower.ends_with(".bmp")
                {
                    let surface = Surface::load_bmp(Path::new(path_str)).unwrap_or_else(|_| panic!("Failed to load BMP from disk '{}'", path_str));
                    let texture = self.1.create_texture_from_surface(&surface).unwrap_or_else(|_| panic!("Failed to create texture from disk '{}'", path_str));
                    let rect = Rect::new(pos.0, pos.1, size.0, size.1);
                    textures.push((texture, rect));
                    continue;
                }
                if lower.ends_with(".png") || lower.ends_with(".jpg") || lower.ends_with(".jpeg")
                {
                    let texture = self.1.load_texture(&mut *path_str).unwrap_or_else(|_| panic!("Failed to load image '{}'", path_str));
                    let rect = Rect::new(pos.0, pos.1, size.0, size.1);
                    textures.push((texture, rect));
                    continue;
                }
            }

            eprintln!("⚠️ Warning: Image '{}' not found (not embedded, not on disk)", path_str);
        }

        textures
    }
}
