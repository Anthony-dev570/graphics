use std::ptr::null;

use mathematics::linear_algebra::vector::types::{Vector2F32, Vector3F32};

use crate::vertex::Vertex;

#[derive(Default, Debug, Clone)]
pub struct PositionUvNormalVertex {
    pub position: Vector3F32,
    pub uv: Vector2F32,
    pub normal: Vector3F32,
}

impl Vertex for PositionUvNormalVertex {
    fn load_attrib_pointers() {
        unsafe {
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * 4 + 2 * 4 + 3 * 4, null());
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 3 * 4 + 2 * 4 + 3 * 4, (3 * 4) as *const _);
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 3 * 4 + 2 * 4 + 3 * 4, (2 * 4 + 3 * 4) as *const _);
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::EnableVertexAttribArray(2);
        }
    }
}