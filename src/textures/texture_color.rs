#[derive(Debug, Clone, Default, Copy)]
#[repr(u32)]
pub enum TextureColor {
    #[default]
    Rgb = gl::RGB,
    Rgba = gl::RGBA,
    Red = gl::RED,
}