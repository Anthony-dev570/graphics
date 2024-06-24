use lazy::lazy::Lazy;
use lazy::lazy_object::LazyObject;
use mathematics::linear_algebra::vector::types::Vector2I32;

use crate::atomic::Atomic;
use crate::frame_buffer::frame_buffer_attachment::FrameBufferAttachment;
use crate::frame_buffer::FrameBuffer;
use crate::render_buffer::buffer_format::BufferFormat;
use crate::render_texture::{RenderTexture, RenderTextureInner, RenderTextureUninitialized};
use crate::textures::image_object::ImageObject;
use crate::textures::texture::Texture;
use crate::textures::texture_color::TextureColor::Rgb;
use crate::textures::texture_settings::TextureSettings;

impl RenderTexture {
    pub fn new(texture_settings: Option<TextureSettings>, buffer_format: Option<BufferFormat>, size: Vector2I32) -> Self {
        Self(Atomic::new(
            LazyObject::new(Lazy::Uninitialized(RenderTextureUninitialized {
                texture_settings: texture_settings.unwrap_or(Self::default_settings()),
                size,
                buffer_format: buffer_format.unwrap_or(BufferFormat::Depth24Stencil8),
            }), Box::new(|unin| {
                let rti = RenderTextureInner {
                    frame_buffer: {
                        let fb = FrameBuffer::default();
                        fb.initialize();
                        fb
                    },
                    texture: {
                        let texture = Texture::default();
                        texture.initialize_2d(
                            unin.texture_settings.clone(),
                            ImageObject::Null {
                                width: unin.size.x(),
                                height: unin.size.y(),
                            }
                        ).unwrap();

                        texture
                    },
                    size: unin.size,
                    buffer_format: unin.buffer_format,
                };
                rti.frame_buffer.attach_texture(FrameBufferAttachment::ColorAttachment(0), rti.texture.clone());
                rti
            }))))
    }

    pub fn default_settings() -> TextureSettings {
        TextureSettings {
            parameters: maplit::hashmap! {
            gl::TEXTURE_MIN_FILTER => gl::LINEAR as i32,
            gl::TEXTURE_MAG_FILTER => gl::LINEAR as i32,
        },
            texture_color: Rgb,
        }
    }
}