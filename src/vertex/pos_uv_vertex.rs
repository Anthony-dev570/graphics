use std::ptr::null;

use mathematics::linear_algebra::vector::types::{Vector2F32, Vector3F32};

use crate::vertex::Vertex;

#[derive(Default, Debug, Clone)]
pub struct PositionUvVertex {
    pub position: Vector3F32,
    pub uv: Vector2F32
}

impl Vertex for PositionUvVertex {
    fn load_attrib_pointers() {
        unsafe {
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * 4 + 2 * 4, null());
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 3 * 4 + 2 * 4, (3 * 4) as *const _);
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
        }
    }
}