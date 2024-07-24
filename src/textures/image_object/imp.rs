use image::{ColorType, DynamicImage, EncodableLayout, ImageResult};
use crate::textures::image_object::ImageObject;
use crate::textures::texture_color::TextureColor;

impl ImageObject {
    pub fn to_image(self) -> ImageResult<DynamicImage> {
        match self {
            ImageObject::Path(path) => {
                image::open(path)
            }
            ImageObject::Image(image) => Ok(image),
            ImageObject::Null {
                width,
                height
            } => Ok(DynamicImage::new(width as u32, height as u32, ColorType::Rgb8)),
            _ => panic!("Not an image type.")
        }
    }

    pub fn as_image(&self) -> ImageResult<DynamicImage> {
        match self {
            ImageObject::Path(path) => {
                image::open(path)
            }
            ImageObject::Image(image) => Ok(image.clone()),
            _ => panic!("Not an image type.")
        }
    }

    pub fn extract(&self, texture_color: TextureColor) -> (i32, i32, Vec<u8>) {
        match self {
            ImageObject::Raw { width, height, pixels } => {
                (*width, *height, pixels.clone())
            }
            _ => {
                let image = self.as_image().unwrap();
                match texture_color {
                    TextureColor::Rgb => {
                        let image = image.to_rgb8();
                        (image.width() as i32, image.height() as i32, image.as_bytes().to_vec())
                    }
                    TextureColor::Rgba => {
                        let image = image.to_rgba8();
                        (image.width() as i32, image.height() as i32, image.as_bytes().to_vec())
                    }
                    TextureColor::Red => {
                        let image = image.to_luma8();
                        (image.width() as i32, image.height() as i32, image.as_bytes().to_vec())
                    }
                }
            }
        }
    }
}