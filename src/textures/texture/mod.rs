use crate::atomic::Atomic;
use crate::textures::texture_settings::TextureSettings;
use crate::textures::texture_type::TextureType;

pub mod imp;

#[allow(dead_code)]
pub struct TextureInner {
    ptr: u32,
    texture_settings: TextureSettings,
    texture_type: TextureType,
}

#[derive(Clone)]
pub struct Texture(Atomic<Option<TextureInner>>);