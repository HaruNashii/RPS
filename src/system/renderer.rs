use std::fmt::Debug;
use std::f64::consts::PI;
use crate::{sdl::sdl3_generators::{GenImage, GenText}, system::{input_handler::InputHandler, page_system::{Button, Page, PageData, PersistentElements}, scene_transition::{SceneTransition, TransitionType} }, AppState};
use sdl3::{pixels::Color, rect::Rect, render::{Canvas, TextureCreator}, ttf::Sdl3TtfContext, video::{Window, WindowContext}};





pub struct Renderer<'a, PageId, ButtonId> 
{ 
    _page_id: Option<PageId>,
    _button_id: Option<ButtonId>,
    pub canvas: &'a mut Canvas<Window>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub ttf_context: &'a Sdl3TtfContext,
    pub font_path: &'a String,
    pub decrease_color_when_selected: Option<(u8, u8, u8)>,
    pub selection_color: Option<(u8, u8, u8, u8)>
}





impl<'a, PageId: Copy + Eq + Debug, ButtonId: Copy + Eq + Debug> Renderer<'a, PageId, ButtonId> 
{
    pub fn new(canvas: &'a mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl3TtfContext, font_path: &'a String, decrease_color_when_selected: Option<(u8, u8, u8)>, selection_color: Option<(u8, u8, u8, u8)>) -> Self { Self{_page_id: None, _button_id: None, canvas, texture_creator, ttf_context, font_path, decrease_color_when_selected, selection_color} }

    pub fn render(&mut self, page_data: &PageData<PageId, ButtonId>, app_state: &mut AppState<PageId, ButtonId>, input_handler: &InputHandler<PageId, ButtonId>) 
    {
        if let Some(page)=&page_data.page_to_render
        {
            if page.has_persistent_elements.is_some()
            { self.render_page(page, Some(&page_data.persistent_elements_to_render), page_data, app_state, input_handler); }
            else
            { self.render_page(page, None, page_data, app_state, input_handler); }
        }
    }

    pub fn render_page(&mut self, page: &Page<PageId, ButtonId>, persistent_elements: Option<&Vec<PersistentElements<PageId, ButtonId>>>, page_data: &PageData<PageId, ButtonId>, app_state: &mut AppState<PageId, ButtonId>, input_handler: &InputHandler<PageId, ButtonId>)
    {
        match page.background_color
        {
            Some(color) => { self.canvas.set_draw_color(color); self.canvas.clear(); }
            None => { self.canvas.set_draw_color(Color::BLACK); self.canvas.clear(); }
        }
        if let Some(persistent) = persistent_elements
        { 
            for persistent_elements in persistent { self.render_elements(Some(page), Some(persistent_elements), page_data, app_state, input_handler); } 
        }
        else 
        { 
            self.render_elements(Some(page), None, page_data, app_state, input_handler); 
        }

        // === scene transition overlay ===
        if let Some(transition) = &mut app_state.scene_transition
        {
            if !transition.has_switched && transition.is_second_stage && let Some(next_page) = transition.next_page.take()
            {
                app_state.current_page = next_page;
                transition.has_switched = true;
            }
            if transition.update(){transition.active=false;}
            else{self.draw_transition_overlay(transition);}
        }

        self.canvas.present();
    }

    pub fn draw_rounded_box(canvas: &mut Canvas<Window>, x: i32, y: i32, w: i32, h: i32, r: i32, color: Color)
    {
        canvas.set_draw_color(color);
        let _ = canvas.fill_rect(Rect::new(x+r,y,(w-2*r) as u32,h as u32));
        for &dx in &[0,w-r]{ let _ = canvas.fill_rect(Rect::new(x+dx,y+r,r as u32,(h-2*r) as u32)); }
        let steps= r * 4;
        for i in 0..=steps
        {
            let theta=(i as f64/steps as f64)*(PI/2.0);
            let ox=(r as f64*theta.cos()).round() as i32;
            let oy=(r as f64*theta.sin()).round() as i32;
            let _ = canvas.draw_line((x+r-ox,y+r-oy),(x+r,y+r-oy));
            let _ = canvas.draw_line((x+w-r-1,y+r-oy),(x+w-r-1+ox,y+r-oy));
            let _ = canvas.draw_line((x+r-ox,y+h-r-1+oy),(x+r,y+h-r-1+oy));
            let _ = canvas.draw_line((x+w-r-1,y+h-r-1+oy),(x+w-r-1+ox,y+h-r-1+oy));
        }
    }

    fn render_elements(&mut self, page: Option<&Page<PageId, ButtonId>>, persistent: Option<&PersistentElements<PageId, ButtonId>>, page_data: &PageData<PageId, ButtonId>, app_state: &AppState<PageId, ButtonId>, input_handler: &InputHandler<PageId, ButtonId>)
    {
        if let Some(page) = page
        {
            if let Some(rects) = &page.rects { for (color,(r,rad)) in rects{ self.canvas.set_draw_color(*color); Self::draw_rounded_box(self.canvas,r.x(),r.y(),r.width() as i32,r.height() as i32,*rad,*color);} }
            if let Some(buttons) = &page.buttons
            { 
                for button in buttons
                { 
                    if button.enabled
                    { 
                        let mut color = button.color;
                        if let Some(button_selected) = input_handler.button_selected && let Some(amount_to_substract) = self.decrease_color_when_selected && button.id == button_selected {color = Color::RGB(button.color.r.saturating_sub(amount_to_substract.0), button.color.g.saturating_sub(amount_to_substract.1), button.color.b.saturating_sub(amount_to_substract.2)); };
                        self.canvas.set_draw_color(color);
                        Self::draw_rounded_box(self.canvas, button.rect.x(), button.rect.y(), button.rect.width() as i32, button.rect.height() as i32, button.radius, color);
                    } 
                } 
            }

            if let Some(text_elements) = &page.texts
            {
                let text_resources = (text_elements,self.texture_creator,self.ttf_context);
                let rendered_texts = text_resources.generate_text(self.font_path);
                let mut rendered_index = 0usize;
                for text_spec in text_elements
                {
                    let text_content = if text_spec.2.is_empty() { " " } else { &text_spec.2 };
                    let font_px = text_spec.0 as f32;
                    let lines: Vec<&str> = text_content.split('\n').collect();
                    let mut first_line_rect: Option<Rect> = None;
                    let mut line_rects: Vec<Rect> = Vec::with_capacity(lines.len());
                    for _ in &lines
                    {
                        let (texture,rect) = &rendered_texts[rendered_index];
                        let _ = self.canvas.copy(texture,None,*rect);
                        if first_line_rect.is_none() { first_line_rect = Some(*rect); }
                        line_rects.push(*rect);
                        rendered_index += 1;
                    }
                    if app_state.capturing_input.0 && let Some(active_button_id) = app_state.capturing_input.1 && let Some(active_input_text) = self.find_active_input_text(page_data,app_state,active_button_id)
                    {
                        let mut target_rect_opt: Option<Rect> = None;
                        if let Some(persistent_elements) = persistent && let Some(button_rect) = self.find_active_button_rect(page,Some(persistent_elements),active_button_id)
                        {
                            for rect in &line_rects { if rect.has_intersection(button_rect) || (rect.y() - button_rect.y()).abs() < 10 { target_rect_opt = Some(*rect); break; } }
                        } 
                        else if let Some(button_rect) = self.find_active_button_rect(page,None,active_button_id)
                        {
                            for rect in &line_rects { if rect.has_intersection(button_rect) || (rect.y() - button_rect.y()).abs() < 10 { target_rect_opt = Some(*rect); break; } }
                        }
                        if target_rect_opt.is_none() && active_input_text == text_content && let Some(rect) = first_line_rect { target_rect_opt = Some(rect); }
                        if let Some(target_rect) = target_rect_opt { self.draw_input_overlay(&target_rect, active_input_text, font_px, input_handler); }
                    }
                }
            }
            if let Some(images)=&page.images
            {
                let image_data=(images,self.texture_creator);
                for (image_texture, image_rect) in image_data.generate_image() { let _ = self.canvas.copy(&image_texture,None,image_rect); }
            }



            // PERSISTENT ELEMENTS
            if let Some(persistent) = persistent
            {
                if let Some(rects) = &persistent.rects { for (color, (rect, border_radius)) in rects { self.canvas.set_draw_color(*color); Self::draw_rounded_box(self.canvas, rect.x(), rect.y(), rect.width() as i32,rect.height() as i32, *border_radius, *color);} }
                if let Some(buttons) = &persistent.buttons 
                { 
                    for button in buttons 
                    { 
                        if button.enabled 
                        { 
                            let mut color = button.color;
                            if let Some(button_selected) = input_handler.button_selected && let Some(amount_to_substract) = self.decrease_color_when_selected && button.id == button_selected {color = Color::RGB(button.color.r.saturating_sub(amount_to_substract.0), button.color.g.saturating_sub(amount_to_substract.1), button.color.b.saturating_sub(amount_to_substract.2)); };
                            self.canvas.set_draw_color(color);
                            Self::draw_rounded_box(self.canvas, button.rect.x(), button.rect.y(), button.rect.width() as i32, button.rect.height() as i32, button.radius, color);
                        } 
                    } 
                }
                if let Some(texts) = &persistent.texts 
                {
                    let requisites = (texts, self.texture_creator, self.ttf_context);
                    for tuple in requisites.generate_text(self.font_path) { self.canvas.copy(&tuple.0, None, tuple.1).unwrap_or_else(|err| {println!("text creator gives an error \nerror: {}\n", err);}); }
                }
                if let Some(images) = &persistent.images
                {
                    let image_data = (images, self.texture_creator);
                    for (image_texture, image_rect) in image_data.generate_image(){ let _ = self.canvas.copy(&image_texture,None,image_rect); }
                }
            }
        }

    }

    pub fn find_active_input_text<'p>(&self, data: &'p PageData<PageId, ButtonId>, app: &AppState<PageId, ButtonId>, button_active: ButtonId) -> Option<&'p str>
    {
        let current_page = app.current_page;
        for (page_id, button_id, string) in &data.vec_user_input{ if *page_id == current_page && *button_id == button_active { return Some(string.as_str()); } }
        None
    }

    pub fn button_matches<T:Copy+Eq+Debug>(button: &Button<T>, a: T) -> bool { button.id == a }

    fn find_active_button_rect(&self, page: &Page<PageId, ButtonId>, option_persistent_elements: Option<&PersistentElements<PageId, ButtonId>>, active: ButtonId) -> Option<Rect>
    {
        if let Some(vec_of_buttons) = &page.buttons { for button in vec_of_buttons { if Self::button_matches(button,active) { return Some(button.rect); } } }
        if let Some(persistent_elements) = option_persistent_elements && let Some(vec_of_buttons) = &persistent_elements.buttons { for button in vec_of_buttons { if Self::button_matches(button, active){ return Some(button.rect); } } }
        None
    }

    fn draw_input_overlay(&mut self,text_rect:&Rect,text_content:&str,font_px:f32,input_state:&InputHandler<PageId,ButtonId>)
    {
        let horizontal_padding = 0;
        let text_start_x = text_rect.x() + horizontal_padding;
        let font = self.ttf_context.load_font(self.font_path, font_px).expect("Failed to load font for input overlay");
        let text_height = font.height() as u32;
        let baseline_y=text_rect.y();
        let character_boundaries:Vec<usize>=text_content.char_indices().map(|(i,_)|i).chain(std::iter::once(text_content.len())).collect();
    
        let measure_text_x = |char_count:usize| -> i32
        {
            let end_index = if char_count<character_boundaries.len() { character_boundaries[char_count] } else { text_content.len() };
            let substring = &text_content[..end_index];
            let (width, _) = font.size_of(substring).unwrap_or((0,0));
            text_start_x + width as i32
        };
    
        let total_chars = text_content.chars().count();
        let clamp_to_text = |n:usize| n.min(total_chars);
    
        let (selection_start, selection_end) = match input_state.text_selection_range
        {
            Some((a,b))=>
            {
                let a = clamp_to_text(a);
                let b = clamp_to_text(b);
                (a.min(b),a.max(b))
            }
            None=>
            {
                let pos = input_state.cursor_position.min(total_chars);
                (pos,pos)
            }
        };
    
        // Draw selection or caret
        if selection_start != selection_end
        {
            let x1 = measure_text_x(selection_start);
            let x2 = measure_text_x(selection_end);
            let selection_width = (x2-x1).max(1) as u32;
            let selection_color = self.selection_color.unwrap_or((0, 0, 255, 125));
            self.canvas.set_draw_color(Color::RGBA(selection_color.0, selection_color.1, selection_color.2, selection_color.3));
            let _ = self.canvas.fill_rect(Rect::new(x1,baseline_y,selection_width,text_height));
        }
        else
        {
            let caret_x = measure_text_x(selection_start);
            self.canvas.set_draw_color(Color::RGB(255,255,255));
            let _ = self.canvas.fill_rect(Rect::new(caret_x,baseline_y,2,text_height));
        }
    }

    pub fn draw_transition_overlay(&mut self,transition: &SceneTransition<PageId>)
    {
        match transition.transition_type
        {
            TransitionType::Fade(alpha)=>
            {
                let a=(alpha*255.0)as u8;
                self.canvas.set_draw_color(Color::RGBA(0,0,0,a));
                let _=self.canvas.fill_rect(None);
            },
            TransitionType::Slide(_) => {},
            _=>{}
        }
    }

}

