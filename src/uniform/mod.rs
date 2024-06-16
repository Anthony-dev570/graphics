use crate::graphics_pointer::GraphicsPointer;

pub mod imp;

pub trait Uniform<T>: GraphicsPointer<T> {
    fn bind_uniform(&self, location: i32);
}