use gl::TEXTURE_2D;
use image::ImageResult;
use crate::atomic::Atomic;
use crate::textures::image_object::ImageObject;
use crate::textures::texture::{Texture, TextureInner, TextureSettings};
use crate::textures::texture_color::TextureColor;
use crate::textures::texture_type::TextureType;

impl Texture {
    pub fn new(
        settings: TextureSettings,
        image_object: ImageObject,
    ) -> ImageResult<Texture> {
        let out = Self::default();
        out.initialize_2d(settings, image_object)?;
        Ok(out)
    }

    pub fn initialize_2d(&self, texture_settings: TextureSettings, image_object: ImageObject) -> ImageResult<()> {
        let mut me = self.0.0.lock().unwrap();
        if me.is_some() {
            return Ok(());
        }
        let (width, height, buffer) = match image_object {
            ImageObject::Raw { width, height, pixels } => (width, height, pixels),
            _ => {
                let image = image_object.to_image()?;

                {
                    let (width, height) = (image.width(), image.height());
                    let pixels = match texture_settings.texture_color {
                        TextureColor::Rgb => image.to_rgb8().to_vec(),
                        TextureColor::Rgba => image.to_rgba8().to_vec(),
                        TextureColor::Red => image.to_luma8().to_vec()
                    };
                    (width as i32, height as i32, pixels)
                }
            }
        };

        let internal_color = texture_settings.clone().texture_color as u32 as i32;
        unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(TEXTURE_2D, texture);

            gl::TexImage2D(
                TEXTURE_2D,
                0,
                internal_color,
                width,
                height,
                0,
                internal_color as u32,
                gl::UNSIGNED_BYTE,
                buffer.as_ptr() as *const _
            );

            for (k, v) in &texture_settings.parameters {
                gl::TexParameteri(TEXTURE_2D, *k, *v);
            }

            *me = Some(TextureInner {
                ptr: texture,
                texture_settings,
                texture_type: TextureType::Texture2D,
            });
        }

        Ok(())
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(self.texture_type() as u32, self.id());
        }
    }

    pub fn id(&self) -> u32 {
        self.0.0.lock().unwrap().as_ref().unwrap().ptr
    }

    pub fn texture_type(&self) -> TextureType {
        self.0.0.lock().unwrap().as_ref().unwrap().texture_type
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self(Atomic::default())
    }
}