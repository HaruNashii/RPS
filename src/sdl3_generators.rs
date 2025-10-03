use std::fs;
use fontconfig::Fontconfig;
use sdl3::
{
    rect::Rect,
    pixels::Color,
    render::{Texture, TextureCreator},
    video::WindowContext,
    image::LoadTexture,
};





pub trait GenText { fn generate_text(&self) -> Vec<(Texture<'_>, Rect)>; }
impl GenText for ( &Vec<(f64, (i32, i32), String, Color)>, &TextureCreator<WindowContext> )
{
    fn generate_text(&self) -> Vec<(Texture<'_>, Rect)>
    {
        let mut vector_to_send = Vec::new();

        for font_content in self.0
        {
            let text_content = if font_content.2.is_empty() { &" ".to_string() } else { &font_content.2 };
            let font_config = Fontconfig::new().unwrap();
            let font = font_config.find("JetBrainsMono", Some("Bold")).unwrap();
            let font_path = font.path.display().to_string();
            let ttf_context = sdl3::ttf::init().unwrap();
            let font = ttf_context.load_font(font_path, font_content.0 as f32).unwrap();
            let surface = font.render(text_content).blended(font_content.3).unwrap();
            let texture = self.1.create_texture_from_surface(&surface).unwrap();
            let rect = Rect::new(font_content.1.0, font_content.1.1, surface.width(), surface.height());

            vector_to_send.push( (texture, rect) );
        }

        vector_to_send
    }
}





pub trait GenImage { fn generate_image(&self) -> Vec<(Texture<'_>, Rect)>; }
impl GenImage for ( &Vec<((i32, i32), (u32, u32), &str)>, &TextureCreator<WindowContext>)
{
    fn generate_image(&self) -> Vec<(Texture<'_>, Rect)>
    {
        let mut new_vec = Vec::new();

        for text_infos in self.0
        {
            if fs::exists(text_infos.2).unwrap()
            {
                let texture = self.1.load_texture(text_infos.2).unwrap();
                let rect = Rect::new(text_infos.0.0, text_infos.0.1, text_infos.1.0, text_infos.1.1);
                new_vec.push((texture, rect));
            }
            else 
            {
                println!("Warning!!!!! Image File '{}' Doesn't Exist", text_infos.2);
            }
        }

        new_vec
    }
}
