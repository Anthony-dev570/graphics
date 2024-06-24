use lazy::lazy_object::LazyObject;
use mathematics::linear_algebra::vector::types::Vector2I32;

use crate::atomic::Atomic;
use crate::frame_buffer::FrameBuffer;
use crate::program::{Program, ProgramInner};
use crate::render_buffer::buffer_format::BufferFormat;
use crate::textures::texture::Texture;
use crate::textures::texture_settings::TextureSettings;

pub mod imp;

pub struct RenderTextureInner {
    frame_buffer: FrameBuffer,
    texture: Texture,
    size: Vector2I32,
    buffer_format: BufferFormat,
    program_inner: Program
}

pub struct RenderTextureUninitialized {
    texture_settings: TextureSettings,
    size: Vector2I32,
    buffer_format: BufferFormat
}

#[derive(Clone)]
pub struct RenderTexture(Atomic<LazyObject<RenderTextureUninitialized, RenderTextureInner>>);