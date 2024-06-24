use std::collections::HashMap;
use lazy::lazy_object::LazyObject;

use crate::atomic::Atomic;
use crate::frame_buffer::frame_buffer_attachment::FrameBufferAttachment;
use crate::render_buffer::RenderBuffer;
use crate::textures::texture::Texture;

pub mod imp;
pub mod frame_buffer_attachment;

pub struct FrameBufferInner {
    ptr: u32,
    render_buffer: Option<RenderBuffer>,
    textures: HashMap<FrameBufferAttachment, Texture>
}

#[derive(Clone)]
pub struct FrameBuffer(Atomic<LazyObject<(), FrameBufferInner>>);