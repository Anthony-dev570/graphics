use std::mem::size_of;
use std::ptr::null;

use mathematics::linear_algebra::vector::types::{Vector2F32, Vector3F32};

use crate::vertex::Vertex;

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct PositionUvVertex {
    pub position: Vector3F32,
    pub uv: Vector2F32
}

impl Vertex for PositionUvVertex {
    fn load_attrib_pointers() {
        unsafe {
            let stride = (size_of::<Vector3F32>() + size_of::<Vector2F32>()) as i32;
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, null());
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, size_of::<Vector3F32>() as *const _);
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
        }
    }
}