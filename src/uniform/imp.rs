use mathematics::linear_algebra::matrix::Matrix;
use mathematics::linear_algebra::vector::types::{Vector2, Vector3};
use crate::graphics_pointer::GraphicsPointer;
use crate::uniform::Uniform;

pub(crate) static mut LOGGING: bool = false;

impl Uniform<f32> for Vector3<f32> {
    fn bind_uniform(&self, location: i32) {
        unsafe {
            let ptr=  self.as_ptr();
            if LOGGING {
                println!("Bind Vector3F32: [location: {}, ptr: {:?}]", location, ptr);
            }
            gl::Uniform3fv(location, 1, ptr);
        }
    }
}

impl Uniform<i32> for Vector3<i32> {
    fn bind_uniform(&self, location: i32) {
        unsafe {
            let ptr=  self.as_ptr();
            if LOGGING {
                println!("Bind Vector3I32: [location: {}, ptr: {:?}]", location, ptr);
            }
            gl::Uniform3iv(location, 1, ptr);
        }
    }
}

impl Uniform<f32> for Matrix<2, 2, f32> {
    fn bind_uniform(&self, location: i32) {
        unsafe {
            let ptr=  self.as_ptr();
            if LOGGING {
                println!("Bind Mat2F32: [location: {}, ptr: {:?}]", location, ptr);
            }
            gl::UniformMatrix2fv(location, 1, gl::FALSE, ptr);
        }
    }
}

impl Uniform<f32> for Matrix<3, 3, f32> {
    fn bind_uniform(&self, location: i32) {
        unsafe {
            let ptr=  self.as_ptr();
            if LOGGING {
                println!("Bind Mat3F32: [location: {}, ptr: {:?}]", location, ptr);
            }
            gl::UniformMatrix3fv(location, 1, gl::FALSE, ptr);
        }
    }
}

impl Uniform<f32> for Matrix<4, 4, f32> {
    fn bind_uniform(&self, location: i32) {
        unsafe {
            let ptr=  self.as_ptr();
            if LOGGING {
                println!("Bind Mat4F32: [location: {}, ptr: {:?}]", location, ptr);
            }
            gl::UniformMatrix4fv(location, 1, gl::FALSE, ptr);
        }
    }
}

impl Uniform<f32> for Vector2<f32> {
    fn bind_uniform(&self, location: i32) {
        unsafe {
            let ptr=  self.as_ptr();
            if LOGGING {
                println!("Bind Vector2F32: [location: {}, ptr: {:?}]", location, ptr);
            }
            gl::Uniform2fv(location, 1, ptr);
        }
    }
}

impl Uniform<i32> for Vector2<i32> {
    fn bind_uniform(&self, location: i32) {
        unsafe {
            let ptr=  self.as_ptr();
            if LOGGING {
                println!("Bind Vector2I32: [location: {}, ptr: {:?}]", location, ptr);
            }
            gl::Uniform2iv(location, 1, ptr);
        }
    }
}

impl GraphicsPointer<i32> for i32 {
    fn as_ptr(&self) -> *const i32 {
        self as *const _
    }
}

impl Uniform<i32> for i32 {
    fn bind_uniform(&self, location: i32) {
        unsafe {
            if LOGGING {
                println!("Bind i32: [location: {}, value: {}]", location, self);
            }
            gl::Uniform1i(location, *self);
        }
    }
}

impl GraphicsPointer<f32> for f32 {
    fn as_ptr(&self) -> *const f32 {
        self as *const _
    }
}

impl Uniform<f32> for f32 {
    fn bind_uniform(&self, location: i32) {
        unsafe {
            if LOGGING {
                println!("Bind f32: [location: {}, value: {}]", location, self);
            }
            gl::Uniform1f(location, *self);
        }
    }
}