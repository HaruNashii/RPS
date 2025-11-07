use crate::{
    AppState, Button, PersistentElements,
    sdl::sdl3_generators::{GenerateImage, GenerateText},
    system::{
        input_handler::InputHandler,
        page_system::{Page, PageData},
        scene_transition::{SceneTransition, TransitionType}
    }
};
use include_dir::Dir;
use sdl3::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::Sdl3TtfContext,
    video::{Window, WindowContext}
};
use std::f64::consts::PI;

/// Renderer Function That Holds The Necessary Data To Render Pages And Transitions
pub struct Renderer<'a, PageId, ButtonId>
{
    pub canvas: Canvas<Window>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub ttf_context: &'a Sdl3TtfContext,
    pub font_path: &'a String,
    pub decrease_color_when_selected: Option<(u8, u8, u8)>,
    pub selection_color: Option<(u8, u8, u8, u8)>,
    pub assets_dir: Option<&'a Dir<'a>>,

    cached_outgoing_page: Option<Page<PageId, ButtonId>>,
    cached_page_data_ptr: *const PageData<PageId, ButtonId>,
    cached_input_handler_ptr: *const InputHandler<PageId, ButtonId>
}

/// RendererConfig Holds The Necessary Configs And Modules
/// To Have The Renderer Working Perfectly
pub struct RendererConfig<'a>
{
    pub canvas: Canvas<Window>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub ttf_context: &'a Sdl3TtfContext,
    pub font_path: &'a String,
    pub decrease_color_when_selected: Option<(u8, u8, u8)>,
    pub selection_color: Option<(u8, u8, u8, u8)>,
    pub assets_dir: Option<&'a Dir<'a>>
}

impl<'a, PageId: Copy + Eq, ButtonId: Copy + Eq> Renderer<'a, PageId, ButtonId>
{
    /// Create And Setup The Renderer
    pub fn new(render_config: RendererConfig<'a>) -> Self
    {
        Self { canvas: render_config.canvas, texture_creator: render_config.texture_creator, ttf_context: render_config.ttf_context, font_path: render_config.font_path, decrease_color_when_selected: render_config.decrease_color_when_selected, selection_color: render_config.selection_color, assets_dir: render_config.assets_dir, cached_outgoing_page: None, cached_page_data_ptr: std::ptr::null(), cached_input_handler_ptr: std::ptr::null() }
    }

    /// Main render entry point. Draws the current page and applies transition overlay if any.
    pub fn render(&mut self, page_data: &PageData<PageId, ButtonId>, app_state: &mut AppState<PageId, ButtonId>, input_handler: &InputHandler<PageId, ButtonId>)
    {
        let page = &mut page_data.page_to_render.clone().unwrap();
        if let Some(bg) = page.background_color
        {
            self.canvas.set_draw_color(bg);
        }
        else
        {
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        }
        self.canvas.clear();
        //has forced_persistent_elements and page don't have persistent elements
        if let Some(forced_persistent_elements) = &page_data.forced_persistent_elements
            && page.has_persistent_elements.is_none()
        {
            self.render_page_base(page, app_state, page_data, Some(forced_persistent_elements.to_vec()), input_handler).unwrap();
        }

        //has forced_persistent_elements and page have persistent elements
        if let Some(forced_persistent_elements) = &page_data.forced_persistent_elements
            && page.has_persistent_elements.is_some()
            && page_data.persistent_elements_to_render.is_some()
        {
            let all_persistent_elements_to_render = [forced_persistent_elements.clone(), page_data.persistent_elements_to_render.clone().unwrap()].concat();
            self.render_page_base(page, app_state, page_data, Some(all_persistent_elements_to_render), input_handler).unwrap();
        }


        //don't have forced_persistent_elements and have persistent_elements
        if page.has_persistent_elements.is_some() && page_data.persistent_elements_to_render.is_some() && page_data.forced_persistent_elements.is_none()
        {
            self.render_page_base(page, app_state, page_data, Some(page_data.persistent_elements_to_render.clone().unwrap()), input_handler).unwrap();
        }
        //don't have forced_persistent_elements and don't have persistent_elements
        else if page_data.forced_persistent_elements.is_none()
        {
            self.render_page_base(page, app_state, page_data, None, input_handler).unwrap();
        }


        if let Some(transition) = &mut app_state.scene_transition
            && matches!(transition.transition_type, TransitionType::Slide(_, _, _))
            && transition.is_second_stage
            && !transition.has_switched
        {
            self.cached_outgoing_page = Some(page.clone());
            self.cached_page_data_ptr = page_data as *const _;
            self.cached_input_handler_ptr = input_handler as *const _;
        }
        let _ = self.scene_transition_overlay(app_state, input_handler);





        self.canvas.present();
    }

    /// Render The Page Without Any Transition
    fn render_page_base(&mut self, page: &mut Page<PageId, ButtonId>, app_state: &mut AppState<PageId, ButtonId>, page_data: &PageData<PageId, ButtonId>, mut persistent_elements: Option<Vec<PersistentElements<PageId, ButtonId>>>, input_handler: &InputHandler<PageId, ButtonId>) -> Result<(), String>
    {
        //NORMAL PAGES
        // RECTS
        if let Some(rects) = &page.rects
        {
            for (color, (r, radius)) in rects
            {
                self.canvas.set_draw_color(*color);
                self.draw_rounded_box(r.x(), r.y(), r.width() as i32, r.height() as i32, *radius, *color);
            }
        }
        // BUTTONS
        if let Some(buttons) = &page.buttons
        {
            for button in buttons
            {
                if button.enabled
                {
                    let mut color = button.color;
                    if let Some(button_selected) = input_handler.button_selected
                        && let Some(amount_to_substract) = self.decrease_color_when_selected
                        && button.id == button_selected
                    {
                        color = Color::RGB(button.color.r.saturating_sub(amount_to_substract.0), button.color.g.saturating_sub(amount_to_substract.1), button.color.b.saturating_sub(amount_to_substract.2));
                    };
                    self.canvas.set_draw_color(color);
                    self.draw_rounded_box(button.rect.x(), button.rect.y(), button.rect.width() as i32, button.rect.height() as i32, button.radius, color);
                }
            }
        }
        // TEXTS
        if let Some(text_elements) = &mut page.texts.clone()
        {
            let mut text_resources = (&mut text_elements.clone(), self.texture_creator, self.ttf_context);
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
                    let (texture, rect) = &rendered_texts[rendered_index];
                    let _ = self.canvas.copy(texture, None, *rect);
                    if first_line_rect.is_none()
                    {
                        first_line_rect = Some(*rect);
                    }
                    line_rects.push(*rect);
                    rendered_index += 1;
                }
                if app_state.capturing_input.0
                    && let Some(active_button_id) = app_state.capturing_input.1
                    && let Some(active_input_text) = self.find_active_input_text(page_data, app_state, active_button_id)
                {
                    let mut target_rect_opt: Option<Rect> = None;
                    if let Some(ref received_persistent) = persistent_elements
                    {
                        for persistent in received_persistent
                        {
                            if let Some(button_rect) = self.find_active_button_rect(page, Some(persistent), active_button_id)
                            {
                                for rect in &line_rects
                                {
                                    if rect.has_intersection(button_rect) || (rect.y() - button_rect.y()).abs() < 10
                                    {
                                        target_rect_opt = Some(*rect);
                                        break;
                                    }
                                }
                            }
                            else if let Some(button_rect) = self.find_active_button_rect(page, None, active_button_id)
                            {
                                for rect in &line_rects
                                {
                                    if rect.has_intersection(button_rect) || (rect.y() - button_rect.y()).abs() < 10
                                    {
                                        target_rect_opt = Some(*rect);
                                        break;
                                    }
                                }
                            }
                        }
                        if target_rect_opt.is_none()
                            && active_input_text == text_content
                            && let Some(rect) = first_line_rect
                        {
                            target_rect_opt = Some(rect);
                        }
                        if let Some(target_rect) = target_rect_opt
                        {
                            self.draw_input_overlay(&target_rect, active_input_text, font_px, input_handler);
                        }
                    }
                }
            }
        }
        // IMAGES
        if let Some(images) = &mut page.images
        {
            let mut image_data = (images, self.texture_creator);
            for (image_texture, image_rect) in image_data.generate_image(self.assets_dir)
            {
                let _ = self.canvas.copy(&image_texture, None, image_rect);
            }
        }

        if let Some(pe_vec) = &page_data.persistent_elements_to_render
        {
            for vec_of_persistent_elements in pe_vec
            {
                if let Some(background_color) = vec_of_persistent_elements.background_color
                {
                    let background_rect = Rect::new(0, 0, 20000, 2000);
                    self.canvas.set_draw_color(background_color);
                    self.canvas.fill_rect(background_rect).unwrap();
                };
            }
        }
        if let Some(pe_vec) = &page_data.forced_persistent_elements
        {
            for vec_of_persistent_elements in pe_vec
            {
                if let Some(background_color) = vec_of_persistent_elements.background_color
                {
                    let background_rect = Rect::new(0, 0, 20000, 2000);
                    self.canvas.set_draw_color(background_color);
                    self.canvas.fill_rect(background_rect).unwrap();
                };
            }
        }


        // PERSISTENT ELEMENTS
        if let Some(vec_page) = &mut persistent_elements
        {
            for page in vec_page
            {
                // RECTS
                if let Some(rects) = &page.rects
                {
                    for (color, (r, radius)) in rects
                    {
                        self.canvas.set_draw_color(*color);
                        self.draw_rounded_box(r.x(), r.y(), r.width() as i32, r.height() as i32, *radius, *color);
                    }
                }
                // BUTTONS
                if let Some(buttons) = &page.buttons
                {
                    for button in buttons
                    {
                        if button.enabled
                        {
                            let mut color = button.color;
                            if let Some(button_selected) = input_handler.button_selected
                                && let Some(amount_to_substract) = self.decrease_color_when_selected
                                && button.id == button_selected
                            {
                                color = Color::RGBA(button.color.r.saturating_sub(amount_to_substract.0), button.color.g.saturating_sub(amount_to_substract.1), button.color.b.saturating_sub(amount_to_substract.2), 255);
                            };
                            self.canvas.set_draw_color(color);
                            self.draw_rounded_box(button.rect.x(), button.rect.y(), button.rect.width() as i32, button.rect.height() as i32, button.radius, color);
                        }
                    }
                }
                // TEXTS
                if let Some(texts) = &mut page.texts
                {
                    let mut requisites = (texts, self.texture_creator, self.ttf_context);
                    for tuple in requisites.generate_text(self.font_path)
                    {
                        self.canvas.copy(&tuple.0, None, tuple.1).unwrap_or_else(|err| {
                            println!("text creator gives an error \nerror: {}\n", err);
                        });
                    }
                }
                // IMAGES
                if let Some(images) = &mut page.images
                {
                    let mut image_data = (images, self.texture_creator);
                    for (image_texture, image_rect) in image_data.generate_image(self.assets_dir)
                    {
                        let _ = self.canvas.copy(&image_texture, None, image_rect);
                    }
                }
            }
        }


        Ok(())
    }

    /// Handle Scene Transition Logic
    fn scene_transition_overlay(&mut self, app_state: &mut AppState<PageId, ButtonId>, input_handler: &InputHandler<PageId, ButtonId>) -> Result<(), String>
    {
        if let Some(transition) = &mut app_state.scene_transition
        {
            if !transition.has_switched
                && transition.is_second_stage
                && let Some(next_page) = transition.next_page.take()
            {
                app_state.current_page = next_page;
                transition.has_switched = true;
            }
            let finished = transition.update();
            self.draw_transition_overlay(transition, input_handler)?;
            if finished
            {
                app_state.scene_transition = None;
                self.cached_outgoing_page = None;
                self.cached_page_data_ptr = std::ptr::null();
                self.cached_input_handler_ptr = std::ptr::null();
            }
        }
        Ok(())
    }

    /// Draw Transition or call a helper that Draw the Transition
    fn draw_transition_overlay(&mut self, transition: &SceneTransition<PageId>, input_handler: &InputHandler<PageId, ButtonId>) -> Result<(), String>
    {
        match transition.transition_type
        {
            TransitionType::None =>
            {}
            TransitionType::Fade(alpha) =>
            {
                let a = (alpha.clamp(0.0, 1.0) * 255.0) as u8;
                self.canvas.set_draw_color(Color::RGBA(0, 0, 0, a));
                self.canvas.fill_rect(None).ok();
            }
            TransitionType::Slide(progress_state, side, speed) =>
            {
                let mut speed_x = 0;
                let mut speed_y = 0;
                match side
                {
                    0 => speed_y = speed,
                    1 => speed_y = -speed,
                    2 => speed_x = speed,
                    3 => speed_x = -speed,
                    _ => speed_y = speed
                }

                let direction_x = (progress_state.clamp(0.0, 1.0) * (speed_x as f32)) as i32;
                let direction_y = (progress_state.clamp(0.0, 1.0) * (speed_y as f32)) as i32;

                // Draw the cached outgoing page on top, shifted to the right.
                if let Some(old_page) = &mut self.cached_outgoing_page.clone()
                {
                    // SAFETY: captured in the same frame from valid references.
                    self.render_page_with_x_offset(old_page, input_handler, direction_x, direction_y)?;
                }

                // If progress hit the end, clear the cache (final cleanup also happens on finished)
                if progress_state >= 0.99
                {
                    self.cached_outgoing_page = None;
                    self.cached_page_data_ptr = std::ptr::null();
                    self.cached_input_handler_ptr = std::ptr::null();
                }
            }
        }
        Ok(())
    }

    /// Draw a full page but shifted by `direction_x` pixels on X (used for sliding transition).
    fn render_page_with_x_offset(&mut self, page: &mut Page<PageId, ButtonId>, input_handler: &InputHandler<PageId, ButtonId>, direction_x: i32, direction_y: i32) -> Result<(), String>
    {
        // NORMAL PAGES
        // RECTS
        if let Some(rects) = &page.rects
        {
            for (color, (r, radius)) in rects
            {
                self.canvas.set_draw_color(*color);
                self.draw_rounded_box(r.x() + direction_x, r.y() + direction_y, r.width() as i32, r.height() as i32, *radius, *color);
            }
        }
        // BUTTONS
        if let Some(buttons) = &page.buttons
        {
            for button in buttons
            {
                if button.enabled
                {
                    let mut color = button.color;
                    if let Some(button_selected) = input_handler.button_selected
                        && let Some(amount_to_substract) = self.decrease_color_when_selected
                        && button.id == button_selected
                    {
                        color = Color::RGB(button.color.r.saturating_sub(amount_to_substract.0), button.color.g.saturating_sub(amount_to_substract.1), button.color.b.saturating_sub(amount_to_substract.2));
                    };
                    self.canvas.set_draw_color(color);
                    self.draw_rounded_box(button.rect.x() + direction_x, button.rect.y() + direction_y, button.rect.width() as i32, button.rect.height() as i32, button.radius, color);
                }
            }
        }
        // TEXTS
        if let Some(texts) = &mut page.texts
        {
            for text in &mut *texts
            {
                text.1.0 += direction_x;
                text.1.1 += direction_y;
            }
            let mut requisites = (texts, self.texture_creator, self.ttf_context);
            for tuple in requisites.generate_text(self.font_path)
            {
                self.canvas.copy(&tuple.0, None, tuple.1).unwrap_or_else(|err| {
                    println!("text creator gives an error \nerror: {}\n", err);
                });
            }
        }
        // IMAGES
        if let Some(images) = &mut page.images
        {
            for image in &mut *images
            {
                image.0.0 += direction_x;
                image.0.1 += direction_y;
            }
            let mut image_data = (images, self.texture_creator);
            for (image_texture, image_rect) in image_data.generate_image(self.assets_dir)
            {
                let _ = self.canvas.copy(&image_texture, None, image_rect);
            }
        }

        Ok(())
    }

    // ===================
    // Minimal utilities
    // ===================
    // Draw radius in the border of the rects and buttons
    pub fn draw_rounded_box(&mut self, x: i32, y: i32, w: i32, h: i32, r: i32, color: Color)
    {
        self.canvas.set_draw_color(color);
        let _ = self.canvas.fill_rect(Rect::new(x + r, y, (w - 2 * r) as u32, h as u32));
        for &dx in &[0, w - r]
        {
            let _ = self.canvas.fill_rect(Rect::new(x + dx, y + r, r as u32, (h - 2 * r) as u32));
        }
        let steps = r * 4;
        for i in 0..=steps
        {
            let theta = (i as f64 / steps as f64) * (PI / 2.0);
            let ox = (r as f64 * theta.cos()).round() as i32;
            let oy = (r as f64 * theta.sin()).round() as i32;
            let _ = self.canvas.draw_line((x + r - ox, y + r - oy), (x + r, y + r - oy));
            let _ = self.canvas.draw_line((x + w - r - 1, y + r - oy), (x + w - r - 1 + ox, y + r - oy));
            let _ = self.canvas.draw_line((x + r - ox, y + h - r - 1 + oy), (x + r, y + h - r - 1 + oy));
            let _ = self.canvas.draw_line((x + w - r - 1, y + h - r - 1 + oy), (x + w - r - 1 + ox, y + h - r - 1 + oy));
        }
    }

    /// Find Which Button Is Current Active
    pub fn find_active_input_text<'p>(&self, data: &'p PageData<PageId, ButtonId>, app: &AppState<PageId, ButtonId>, button_active: ButtonId) -> Option<&'p str>
    {
        let current_page = app.current_page;
        for (page_id, button_id, string) in &data.vec_user_input
        {
            if *page_id == current_page && *button_id == button_active
            {
                return Some(string.as_str());
            }
        }
        None
    }

    /// Check If The Button Matches With Other
    pub fn button_matches<T: Copy + Eq>(button: &Button<T>, a: T) -> bool
    {
        button.id == a
    }

    /// Find The Rect Of The Current Active Button
    fn find_active_button_rect(&self, page: &Page<PageId, ButtonId>, option_persistent_elements: Option<&PersistentElements<PageId, ButtonId>>, active: ButtonId) -> Option<Rect>
    {
        if let Some(vec_of_buttons) = &page.buttons
        {
            for button in vec_of_buttons
            {
                if Self::button_matches(button, active)
                {
                    return Some(button.rect);
                }
            }
        }
        if let Some(persistent_elements) = option_persistent_elements
            && let Some(vec_of_buttons) = &persistent_elements.buttons
        {
            for button in vec_of_buttons
            {
                if Self::button_matches(button, active)
                {
                    return Some(button.rect);
                }
            }
        }
        None
    }

    /// Draw The Input Box Overlay, like: (Selection, Cursor, Etc...)
    fn draw_input_overlay(&mut self, text_rect: &Rect, text_content: &str, font_px: f32, input_state: &InputHandler<PageId, ButtonId>)
    {
        let horizontal_padding = 0;
        let text_start_x = text_rect.x() + horizontal_padding;
        let font = self.ttf_context.load_font(self.font_path, font_px).expect("Failed to load font for input overlay");
        let text_height = font.height() as u32;
        let baseline_y = text_rect.y();
        let character_boundaries: Vec<usize> = text_content.char_indices().map(|(i, _)| i).chain(std::iter::once(text_content.len())).collect();

        let measure_text_x = |char_count: usize| -> i32 {
            let end_index = if char_count < character_boundaries.len() { character_boundaries[char_count] } else { text_content.len() };
            let substring = &text_content[..end_index];
            let (width, _) = font.size_of(substring).unwrap_or((0, 0));
            text_start_x + width as i32
        };

        let total_chars = text_content.chars().count();
        let clamp_to_text = |n: usize| n.min(total_chars);

        let (selection_start, selection_end) = match input_state.text_selection_range
        {
            Some((a, b)) =>
            {
                let a = clamp_to_text(a);
                let b = clamp_to_text(b);
                (a.min(b), a.max(b))
            }
            None =>
            {
                let pos = input_state.cursor_position.min(total_chars);
                (pos, pos)
            }
        };

        // Draw selection or caret
        if selection_start != selection_end
        {
            let x1 = measure_text_x(selection_start);
            let x2 = measure_text_x(selection_end);
            let selection_width = (x2 - x1).max(1) as u32;
            let selection_color = self.selection_color.unwrap_or((0, 0, 255, 125));
            self.canvas.set_draw_color(Color::RGBA(selection_color.0, selection_color.1, selection_color.2, selection_color.3));
            let _ = self.canvas.fill_rect(Rect::new(x1, baseline_y, selection_width, text_height));
        }
        else
        {
            let caret_x = measure_text_x(selection_start);
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            let _ = self.canvas.fill_rect(Rect::new(caret_x, baseline_y, 2, text_height));
        }
    }
}
