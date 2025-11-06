use include_dir::Dir;
use sdl3::{
    image::LoadTexture,
    iostream::IOStream,
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator},
    surface::Surface,
    ttf::Sdl3TtfContext,
    video::WindowContext
};
use std::path::{Path, PathBuf};

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
                    let mut stream = IOStream::from_bytes(file.contents()).expect("Failed to create IOStream from embedded data");
                    let surface = Surface::load_bmp_rw(&mut stream).unwrap_or_else(|_| panic!("Failed to load embedded BMP '{}'", normalized));
                    let texture = self.1.create_texture_from_surface(&surface).unwrap_or_else(|_| panic!("Failed to create texture for '{}'", normalized));
                    let rect = Rect::new(pos.0, pos.1, size.0, size.1);
                    textures.push((texture, rect));
                    continue;
                }

                // === 2️⃣ Recursive lookup by filename only ===
                if let Some(file) = assets.files().find(|f| f.path().file_name() == Path::new(&normalized).file_name())
                {
                    let mut stream = IOStream::from_bytes(file.contents()).expect("Failed to create IOStream from embedded data (search mode)");
                    let surface = Surface::load_bmp_rw(&mut stream).unwrap_or_else(|_| panic!("Failed to load embedded BMP '{}'", normalized));
                    let texture = self.1.create_texture_from_surface(&surface).unwrap_or_else(|_| panic!("Failed to create texture for '{}'", normalized));
                    let rect = Rect::new(pos.0, pos.1, size.0, size.1);
                    textures.push((texture, rect));
                    continue;
                }
            }

            // === 3️⃣ Fallback: load from disk ===
            let lower = path_str.to_lowercase();
            if Path::new(path_str).exists()
            {
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
