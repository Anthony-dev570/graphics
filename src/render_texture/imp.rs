use lazy::lazy::Lazy;
use lazy::lazy_object::LazyObject;
use mathematics::linear_algebra::vector::types::Vector2I32;

use crate::atomic::Atomic;
use crate::frame_buffer::frame_buffer_attachment::FrameBufferAttachment;
use crate::frame_buffer::FrameBuffer;
use crate::program::Program;
use crate::render_buffer::buffer_format::BufferFormat;
use crate::render_texture::{RenderTexture, RenderTextureInner, RenderTextureUninitialized};
use crate::textures::image_object::ImageObject;
use crate::textures::texture::Texture;
use crate::textures::texture_color::TextureColor::Rgb;
use crate::textures::texture_settings::TextureSettings;

const PROGRAM_VS: &'static str = {r#"
    #version 330 core
    layout (location = 0) in vec2 aPos;
    layout (location = 1) in vec2 aTexCoords;

    out vec2 TexCoords;

    void main() {
        gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
        TexCoords = aTexCoords;
    }
"#};

const PROGRAM_FS: &'static str = {r#"
    #version 330 core
    out vec4 FragColor;

    in vec2 TexCoords;

    uniform sampler2D screenTexture;

    void main() {
        FragColor = texture(screenTexture, TexCoords);
    }
"#};

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
                    program_inner: Program::new_from_src(PROGRAM_VS, PROGRAM_FS).unwrap(),
                };
                rti.frame_buffer.attach_texture(FrameBufferAttachment::ColorAttachment(0), rti.texture.clone());
                rti
            }))))
    }

    pub fn bind_texture(&self) {
        self.0.0.lock().unwrap().get().texture.bind();
    }

    pub fn initialize(&self) {
        self.0.0.lock().unwrap().initialize();
    }

    pub fn id(&self) -> u32 {
        self.0.0.lock().unwrap().get().frame_buffer.id()
    }

    pub fn render<F>(&self, draw_fn: F) where F: Fn() {
        let mut me = self.0.0.lock().unwrap();
        let me = me.get();
        me.frame_buffer.bind();
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::DEPTH_TEST);
            draw_fn();
            gl::Disable(gl::DEPTH_TEST);
        }
        me.frame_buffer.unbind();
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