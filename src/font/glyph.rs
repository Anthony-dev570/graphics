use std::collections::HashMap;

use freetype::Face;
use freetype::face::LoadFlag;
use gl::{CLAMP_TO_EDGE, LINEAR, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T};
use mathematics::linear_algebra::vector::types::Vector2I32;

use crate::a_or_b::AOrB;
use crate::graphics_error::GraphicsError;
use crate::texture::{ImageObject, Texture, TextureSettings};
use crate::texture::TextureColor::Red;

static mut GLYPH_PARAMS: AOrB<&'static [(u32, i32)], HashMap<u32, i32>> = AOrB::A(&[
    (TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32),
    (TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32),

    (TEXTURE_MIN_FILTER, LINEAR as i32),
    (TEXTURE_MAG_FILTER, LINEAR as i32)
]);

#[derive(Clone)]
pub struct Glyph {
    texture: Texture,
    size: Vector2I32,
    bearing: Vector2I32,
    advance: i32
}

impl Glyph {
    fn params() -> &'static HashMap<u32, i32> {
        unsafe {
            GLYPH_PARAMS = GLYPH_PARAMS.clone().to_b(|params| {
                params.iter().map(|(k, v)| (*k, *v)).collect::<HashMap<u32, i32>>()
            });
            GLYPH_PARAMS.b().unwrap()
        }
    }

    pub fn load(
        face: &Face,
        char: usize,
    ) -> Result<Self, GraphicsError> {
        face.load_char(char, LoadFlag::RENDER).map_err(|e| GraphicsError::FreeType(e))?;
        let face_glyph = face.glyph();

        let texture = Texture::new(TextureSettings {
            parameters: Self::params().clone(),
            texture_color: Red,
        }, ImageObject::Raw {
            width: face_glyph.bitmap().width(),
            height: face_glyph.bitmap().rows(),
            pixels: face_glyph.bitmap().buffer().to_vec(),
        }).map_err(|e| GraphicsError::ImageError(e))?;

        let size = Vector2I32::new([face_glyph.bitmap().width(), face_glyph.bitmap().rows()]);
        let bearing = Vector2I32::new([
            face_glyph.bitmap_left(), face_glyph.bitmap_top()
        ]);
        let advance = face_glyph.advance().x;

        Ok(Self {
            texture,
            size,
            bearing,
            advance,
        })
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
    pub fn size(&self) -> Vector2I32 {
        self.size
    }
    pub fn bearing(&self) -> Vector2I32 {
        self.bearing
    }
    pub fn advance(&self) -> i32 {
        self.advance
    }
}