use std::ptr::null;

use mathematics::linear_algebra::vector::types::Vector3F32;

use crate::vertex::Vertex;

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct PositionVertex {
    pub position: Vector3F32
}

impl Vertex for PositionVertex {
    fn load_attrib_pointers() {
        unsafe {
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * 4, null());
            gl::EnableVertexAttribArray(0);
        }
    }
}