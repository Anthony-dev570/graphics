use lazy::lazy_object::LazyObject;
use crate::atomic::Atomic;

pub mod imp;
pub mod buffer_format;

pub struct RenderBufferInner(u32);

#[derive(Clone)]
pub struct RenderBuffer(Atomic<LazyObject<(), RenderBufferInner>>);