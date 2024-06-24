use image::DynamicImage;

pub mod imp;

#[derive(Clone)]
pub enum ImageObject {
    Path(String),
    Image(DynamicImage),
    Raw {
        width: i32,
        height: i32,
        pixels: Vec<u8>
    },
    Null {
        width: i32,
        height: i32
    }
}