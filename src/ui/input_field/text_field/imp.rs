use std::fmt::Write;
use bumpalo::Bump;
use mathematics::linear_algebra::matrix::types::Mat4F32;
use mathematics::linear_algebra::vector::types::{Vector2F32, Vector3F32};
use crate::atomic::Atomic;
use crate::font::Font;
use crate::font::font_type::FontType;
use crate::ui::input_field::InputField;
use crate::ui::input_field::text_field::{TextField, TextFilter};
use crate::ui::style::{Style, StyleProperty, StyleValue};
use crate::ui::{FOCUS, send_lost_focus, UI, UI_ELEMENTS};

impl TextField {
    pub fn new_with_buffer<V: ToString>(value: V) -> Self {
        let out = Self::default();
        out.0.0.lock().unwrap().buffer = value.to_string();
        out
    }

    pub fn set_font(&self, font: Option<Font>) {
        self.0.0.lock().unwrap().font = font;
    }

    pub fn with_filter(self, filter: TextFilter) -> Self {
        self.0.0.lock().unwrap().text_filter = Some(filter);
        self
    }

    pub fn filter(&self) -> Option<TextFilter> {
        self.0.0.lock().unwrap().text_filter.clone()
    }

    pub fn buffer(&self) -> String {
        self.0.0.lock().unwrap().buffer.clone()
    }

    fn evaluate(&self) {
        if let Some(filter) = self.filter() {
            if filter == TextFilter::Algebraic {
                let buffer = self.buffer();
                let ctx = aftermath::Context::new();
                let mut bump = Bump::new();
                let expr = aftermath::Expr::parse(&mut bump, &buffer, &[]).unwrap();
                let e = ctx.eval(expr).unwrap();

                self.0.0.lock().unwrap().buffer = e.norm().to_string();
            }
        }
    }
}

impl Default for TextField {
    fn default() -> Self {
        let d = Self(Atomic::default());

        //ui_elements_mut().insert(d.id(), Box::new(d.clone()));
        unsafe {
            UI_ELEMENTS.push(Box::new(d.clone()));
        }

        d
    }
}

impl UI for TextField {
    fn id(&self) -> usize {
        self.0.0.lock().unwrap().id
    }

    fn process_click(&self, _point: Vector2F32, _mouse_button: u32) {
        self.0.0.lock().unwrap().has_error = false;
    }

    fn process_key(&self, key: u32, action: u32) {
        let filter = self.0.0.lock().unwrap().text_filter.clone().unwrap_or(TextFilter::Alphanumeric);
        if action == 1 || action == 2 {
            if key == 259 {
                let buffer = &mut self.0.0.lock().unwrap().buffer;
                if buffer.len() > 0 {
                    buffer.remove(buffer.len() - 1);
                }
            } else {
                let buffer = &mut self.0.0.lock().unwrap().buffer;
                //println!("{key}");
                match key == 257 || key == 335 {
                    true => {
                        match filter {
                            TextFilter::Algebraic => {
                                //self.evaluate();
                            }
                            _ => {}
                        }
                    }
                    false => {
                        let char = match key {
                            320..=329 => {
                                let value = key - 320;

                                value.to_string().chars().nth(0).unwrap()
                            },
                            330 => '.',
                            _ => key as u8 as char
                        };
                        //println!("{}", key);
                        let add = match filter {
                            TextFilter::NumbersOnly => {
                                match char == '.' && buffer.contains(".") {
                                    true => false,
                                    false => char.is_numeric() || char == '.'
                                }
                            }
                            TextFilter::Alphanumeric => char.is_alphanumeric(),
                            TextFilter::AlphaOnly => char.is_alphabetic(),
                            TextFilter::Algebraic => char.is_numeric() || ['.', '/', '+', '-', '*', '(', ')', '{', '}', '[', ']'].contains(&char)
                        };

                        if add {
                            buffer.write_char(char).unwrap()
                        }
                    }
                }
            }
        }
    }

    fn draw(&self, projection: &Mat4F32) {
        let program_vao = Self::bundled();

        let position = self.position();
        let size = self.size();
        let vertices = Self::build_rect3d(position, size);

        program_vao.mesh.set_vertices(&vertices);

        let border_size = 4_f32;

        program_vao.program.bind();
        program_vao.program.bind_uniform("projection", *projection);
        program_vao.program.bind_uniform("radiusX", border_size / 2_f32 / size[0]);
        program_vao.program.bind_uniform("radiusY", border_size / 2_f32 / size[1]);

        let color = match self.has_focus() {
            true => Vector3F32::new([1.0, 0_f32, 0_f32]),
            false => Vector3F32::new([0.0, 0_f32, 0_f32]),
        };

        program_vao.program.bind_uniform("u_color", color);
        program_vao.mesh.draw();

        let text = self.0.0.lock().unwrap().buffer.clone();

        if let Some(font) = &self.0.0.lock().unwrap().font {
            let text_width = font.text_width(&text, FontType::Standard, 12_f32).unwrap();

            //println!("{:?}", position);

            font.render_text(
                text,
                FontType::Standard,
                position + Vector2F32::new([(size[0] / 2_f32) - (text_width / 2_f32), (size[1] / 2_f32) - 6_f32]),
                12_f32,
                Vector3F32::new([1.0, 0_f32, 0_f32]),
                projection,
            );
        }
    }

    fn size(&self) -> Vector2F32 {
        self.0.0.lock().unwrap().size
    }

    fn position(&self) -> Vector2F32 {
        self.0.0.lock().unwrap().position
    }

    fn set_position(&self, position: Vector2F32) {
        self.0.0.lock().unwrap().position = position;
    }

    fn set_size(&self, size: Vector2F32) {
        self.0.0.lock().unwrap().size = size;
    }

    fn style(&self) -> Style {
        self.0.0.lock().unwrap().style.clone()
    }

    fn insert_style_property(&self, property: StyleProperty, value: StyleValue) {
        self.0.0.lock().unwrap().style.put_property(property, value)
    }

    fn request_focus(&self) {
        send_lost_focus();
        unsafe {
            FOCUS = Some(*Box::new(self.id()));
        }
    }

    fn on_lost_focus(&self) {
        self.evaluate();
    }
}

impl InputField<String> for TextField {
    fn value(&self) -> String {
        self.0.0.lock().unwrap().buffer.clone()
    }

    fn to_value(&self) -> Option<String> {
        Some(self.0.0.lock().unwrap().buffer.clone())
    }

    fn caret(&self) -> usize {
        self.0.0.lock().unwrap().caret
    }
}