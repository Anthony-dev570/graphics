use std::collections::HashMap;
use std::path::Path;

use freetype::Library;
use gl::TEXTURE_2D;
use mathematics::linear_algebra::matrix::types::Mat4F32;
use mathematics::linear_algebra::vector::types::{Vector2F32, Vector3F32};

use crate::atomic::Atomic;
use crate::font::{Font, FontHandler, FontInfo, FontInner};
use crate::font::font_type::FontType;
use crate::font::glyph::Glyph;
use crate::graphics_error::GraphicsError;
use crate::program::Program;
use crate::vertex::pos2_uv::Position2Uv;
use crate::vertex_array::VertexArray;

static mut LIBRARY: Option<Library> = None;
static mut VAO_PROGRAM: Option<(VertexArray, Program)> = None;

const VERTEX_SHADER: &'static str = include_str!("../../res/font/font_vs.glsl");
const FRAGMENT_SHADER: &'static str = include_str!("../../res/font/font_fs.glsl");

impl Font {
    pub fn new<N: ToString, P: AsRef<Path> + ToString>(
        name: N,
        font_types: HashMap<FontType, P>,
        render_font_size: u32,
    ) -> Self {
        Self(Atomic::new(FontHandler::Uninitialized {
            name: name.to_string(),
            font_types: font_types.into_iter().map(|(k, v)| (k, v.to_string())).collect::<HashMap<FontType, String>>(),
            render_font_size,
        }))
    }

    pub fn render_size(&self) -> u32 {
        match &*self.0.0.lock().unwrap() {
            FontHandler::Uninitialized { render_font_size, .. } => *render_font_size,
            FontHandler::Initialized(init) => init.render_font_size
        }
    }

    pub fn render_text<T: ToString>(&self, text: T, font_type: FontType, position: Vector2F32, font_size: f32, color: Vector3F32, projection: &Mat4F32) -> Option<()> {
        let (mut x, y) = (position[0], position[1]);

        let (vao, program) = Self::load_vao_and_program();

        program.bind();
        program.bind_uniform("textColor", color);
        program.bind_uniform("projection", *projection);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            vao.bind();

            let characters = self.get_characters(font_type)?;

            let scale = font_size / self.render_size() as f32;

            println!("Scale: {}", scale);

            for c in text.to_string().chars() {
                let glyph = &characters.characters[&c];

                let x_pos = x + glyph.bearing()[0] as f32 * scale;
                let y_pos = y - (glyph.size()[1] as f32 - glyph.bearing()[1] as f32) * scale;

                let w = glyph.size()[0] as f32 * scale;
                let h = glyph.size()[1] as f32 * scale;

                let vertices = [
                    Position2Uv {
                        position: Vector2F32::new([x_pos, y_pos + h]),
                        uv: Default::default(),
                    },
                    Position2Uv {
                        position: Vector2F32::new([x_pos, y_pos]),
                        uv: Vector2F32::new([0_f32, 1_f32]),
                    },
                    Position2Uv {
                        position: Vector2F32::new([x_pos + w, y_pos]),
                        uv: Vector2F32::new([1_f32; 2]),
                    },

                    Position2Uv {
                        position: Vector2F32::new([x_pos, y_pos + h]),
                        uv: Vector2F32::new([0_f32; 2]),
                    },
                    Position2Uv {
                        position: Vector2F32::new([x_pos + w, y_pos]),
                        uv: Vector2F32::new([1_f32; 2]),
                    },
                    Position2Uv {
                        position: Vector2F32::new([x_pos + w, y_pos + h]),
                        uv: Vector2F32::new([1_f32, 0_f32]),
                    },
                ];

                let v = vertices.iter().map(|v| v.position).collect::<Vec<Vector2F32>>();

                println!("{:?}", v);

                glyph.texture().bind();
                vao.set_vertices(&vertices, None);

                gl::DrawArrays(gl::TRIANGLES, 0, 6);

                x += (glyph.advance() >> 6) as f32 * scale;
            }
            gl::BindVertexArray(0);
            gl::BindTexture(TEXTURE_2D, 0);
        }
        Some(())
    }

    pub fn get_characters(&self, font_type: FontType) -> Option<FontInfo> {
        match &*self.0.0.lock().unwrap() {
            FontHandler::Uninitialized { .. } => None,
            FontHandler::Initialized(initialized) => {
                initialized.fonts.get(&font_type).map(|s| s.clone())
            }
        }
    }

    pub fn is_initialized(&self) -> bool {
        match &*self.0.0.lock().unwrap() {
            FontHandler::Uninitialized { .. } => false,
            FontHandler::Initialized(_) => true
        }
    }

    pub fn initialize(&self) -> Result<(), GraphicsError> {
        let mut ptr = self.0.0.lock().unwrap();
        if let FontHandler::Uninitialized { name, font_types, render_font_size } = &*ptr {
            println!("Begin loading font {name}.");
            let mut fonts = HashMap::new();

            unsafe {
                gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
                let library = Self::load_library();

                for (k, v) in font_types {
                    let face = library.new_face(v, 0).map_err(|e| GraphicsError::FreeType(e))?;
                    face.set_pixel_sizes(0, *render_font_size).map_err(|e| GraphicsError::FreeType(e))?;

                    let mut characters = HashMap::new();

                    for i in 0..128 {
                        let glyph = Glyph::load(&face, i)?;
                        characters.insert(i as u8 as char, glyph);
                        println!("Finished loading char: [{} => {:?}]", i, i as u8 as char);
                    }

                    println!("Loaded all glyphs: {}", characters.len());

                    fonts.insert(*k, FontInfo {
                        characters,
                        path: v.to_string(),
                    });
                }
            }

            *ptr = FontHandler::Initialized(FontInner {
                render_font_size: *render_font_size,
                name: name.clone(),
                fonts,
            });

            println!("Assigned font inner.");
        }
        Ok(())
    }

    fn load_vao_and_program() -> &'static (VertexArray, Program) {
        unsafe {
            if let Some(out) = &VAO_PROGRAM {
                return out;
            } else {
                let program = Program::new_from_src(VERTEX_SHADER, FRAGMENT_SHADER).unwrap();
                let vao = VertexArray::default();
                vao.set_vertices::<Position2Uv>(&[], None);
                VAO_PROGRAM = Some((vao, program));
                Self::load_vao_and_program()
            }
        }
    }

    fn load_library() -> &'static Library {
        unsafe {
            match &LIBRARY {
                None => {
                    LIBRARY = Some(Library::init().unwrap());
                    Self::load_library()
                }
                Some(library) => library
            }
        }
    }
}