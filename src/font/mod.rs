use std::collections::HashMap;
use crate::atomic::Atomic;
use crate::font::font_type::FontType;
use crate::font::glyph::Glyph;

pub mod imp;
pub mod glyph;
pub mod font_type;

pub struct FontInner {
    render_font_size: i32,
    name: String,
    fonts: HashMap<FontType, FontInner>,
}

pub struct FontInfo {
    characters: HashMap<char, Glyph>,
    path: String,
}

pub enum FontHandler {
    Uninitialized {
        name: String,
        font_types: HashMap<FontType, String>,
        render_font_size: i32
    },
    Initialized(FontInner)
}

#[derive(Clone)]
pub struct Font(Atomic<FontHandler>);