use std::ptr::null;

use mathematics::linear_algebra::vector::types::Vector2F32;

use crate::vertex::Vertex;

#[derive(Default, Debug, Clone, Copy)]
pub struct Position2Uv {
    pub position: Vector2F32,
    pub uv: Vector2F32
}

impl Vertex for Position2Uv {
    fn load_attrib_pointers() {
        unsafe {
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4 * 4, null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 4 * 4, (2 * 4) as *const _);
            gl::EnableVertexAttribArray(1);
        }
    }
}