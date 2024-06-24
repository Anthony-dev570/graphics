use lazy::lazy::Lazy::Uninitialized;
use lazy::lazy_object::LazyObject;

use crate::atomic::Atomic;
use crate::frame_buffer::{FrameBuffer, FrameBufferInner};
use crate::frame_buffer::frame_buffer_attachment::FrameBufferAttachment;
use crate::render_buffer::RenderBuffer;
use crate::textures::texture::Texture;

impl FrameBuffer {
    pub fn id(&self) -> u32 {
        self.0.0.lock().unwrap().get().ptr
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id());
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn initialize(&self) {
        self.0.0.lock().unwrap().initialize()
    }

    pub fn is_initialized(&self) -> bool {
        self.0.0.lock().unwrap().is_initialized()
    }

    pub fn attach_render_buffer(&self, render_buffer: RenderBuffer) {
        self.bind();
        unsafe {
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                render_buffer.id()
            );
            self.0.0.lock().unwrap().get().render_buffer = Some(render_buffer);
        }
    }

    pub fn attach_texture(&self, attachment: FrameBufferAttachment, texture: Texture) {
        unsafe {
            self.bind();
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                attachment.id(),
                gl::TEXTURE_2D,
                texture.id(),
                0
            );
            self.0.0.lock().unwrap().get().textures.insert(attachment, texture);
        }
    }
}

impl Drop for FrameBufferInner {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.ptr);
        }
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self(Atomic::new(LazyObject::new(Uninitialized(()), Box::new(|_| {
            FrameBufferInner {
                ptr: crate::ffi::gen_frame_buffer(),
                render_buffer: None,
                textures: Default::default(),
            }
        }))))
    }
}