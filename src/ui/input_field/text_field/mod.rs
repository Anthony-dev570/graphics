use mathematics::linear_algebra::vector::types::Vector2F32;
use crate::atomic::Atomic;

pub mod imp;

#[derive(Default)]
pub struct TextFieldInner {
    buffer: String,
    caret: usize,
    position: Vector2F32,
    size: Vector2F32
}

#[derive(Clone)]
pub struct TextField(Atomic<TextFieldInner>);