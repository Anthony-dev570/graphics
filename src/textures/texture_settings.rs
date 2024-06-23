use std::collections::HashMap;
use crate::textures::texture_color::TextureColor;

#[derive(Default, Clone)]
pub struct TextureSettings {
    pub parameters: HashMap<u32, i32>,
    pub texture_color: TextureColor,
}