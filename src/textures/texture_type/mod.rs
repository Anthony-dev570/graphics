use gl::{TEXTURE_2D, TEXTURE_CUBE_MAP};

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum TextureType {
    Texture2D = TEXTURE_2D,
    TextureCubeMap = TEXTURE_CUBE_MAP
}
