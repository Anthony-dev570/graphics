use crate::graphics_pointer::GraphicsPointer;

pub mod imp;
pub mod uniform_struct;

pub trait Uniform<T>: GraphicsPointer<T> {
    fn bind_uniform(&self, location: i32);

    fn enable_logging(logging: bool) {
        unsafe {
            imp::LOGGING = logging;
        }
    }
}