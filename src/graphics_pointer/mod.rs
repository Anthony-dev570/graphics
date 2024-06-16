use std::ffi::c_void;

pub mod imp;

pub trait GraphicsPointer<T> {
    fn as_ptr(&self) -> *const T;
    fn as_c_ptr(&self) -> *const c_void {
        self.as_ptr() as *const _
    }
}