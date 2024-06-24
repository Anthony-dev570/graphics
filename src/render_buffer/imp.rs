use lazy::lazy::Lazy;
use lazy::lazy_object::LazyObject;
use mathematics::linear_algebra::vector::types::Vector2I32;

use crate::atomic::Atomic;
use crate::ffi::gen_render_buffer;
use crate::render_buffer::{RenderBuffer, RenderBufferInner};
use crate::render_buffer::buffer_format::BufferFormat;

impl RenderBuffer {
    pub fn id(&self) -> u32 {
        self.0.0.lock().unwrap().get().0
    }

    pub fn initialize(&self) {
        self.0.0.lock().unwrap().initialize()
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.id());
        }
    }

    pub fn buffer_storage(&self, buffer_format: BufferFormat, size: Vector2I32) {
        unsafe { gl::RenderbufferStorage(gl::RENDERBUFFER, buffer_format as u32, size.x(), size.y()); }
    }
}

impl Default for RenderBuffer {
    fn default() -> Self {
        Self(Atomic::new(LazyObject::new(Lazy::Uninitialized(()), Box::new(|_| {
            RenderBufferInner(gen_render_buffer())
        }))))
    }
}

impl Drop for RenderBufferInner {
    fn drop(&mut self) {
        unsafe { gl::DeleteRenderbuffers(1, &self.0); }
    }
}