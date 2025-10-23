use std::fs;
use sdl3::
{
    image::LoadTexture,
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator},
    ttf::Sdl3TtfContext,
    video::WindowContext,
};





pub trait GenText { fn generate_text(&self, font_path: &String) -> Vec<(Texture<'_>, Rect)>; }
impl GenText for (&Vec<(f64, (i32, i32), String, Color)>, &TextureCreator<WindowContext>, &Sdl3TtfContext) 
{
    fn generate_text(&self, font_path: &String) -> Vec<(Texture<'_>, Rect)> 
    {
        let mut vector_to_send = Vec::new();
        for font_content in self.0 
        { 
            let text_content = if font_content.2.is_empty() { " " } else { &font_content.2 };
            let font = self.2.load_font(font_path, font_content.0 as f32).expect("Failed to load font");

            let lines: Vec<&str> = text_content.split('\n').collect();
            let mut current_y = font_content.1.1;
            for line in lines 
            {
                let render_text = if line.is_empty() { " " } else { line };
                let surface = font.render(render_text).blended(font_content.3).expect("Failed to blend font");
                let texture = self.1.create_texture_from_surface(&surface).expect("Failed to create font texture");
                let rect = Rect::new(font_content.1.0, current_y, surface.width(), surface.height(),);
                vector_to_send.push((texture, rect));
                current_y += surface.height() as i32;
            }
            drop(font);
        }

        vector_to_send
    }
}





pub trait GenImage { fn generate_image(&self) -> Vec<(Texture<'_>, Rect)>; }
impl GenImage for (&Vec<((i32, i32), (u32, u32), String)>, &TextureCreator<WindowContext>) 
{
    fn generate_image(&self) -> Vec<(Texture<'_>, Rect)> 
    {
        let mut new_vec = Vec::new();
        for text_infos in self.0 
        {
            if fs::exists(text_infos.2.clone()).expect("Failed To Check If File Exist") 
            {
                let texture = self.1.load_texture(text_infos.2.clone()).expect("Failed To Create Image Texture");
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
