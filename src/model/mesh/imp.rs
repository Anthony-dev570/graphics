use std::ptr::null;
use crate::atomic::Atomic;
use crate::model::mesh::{Mesh, MeshInner};
use crate::vertex::Vertex;
use crate::vertex_array::VertexArray;

impl<V: Vertex + Clone> Mesh<V> {
    pub fn initialized(&self) -> bool {
        self.0.0.lock().unwrap().is_some()
    }
    pub fn initialize(
        &self,
        vertices: &[V],
        indices: Option<&[u32]>,
    ) {
        if !self.initialized() {
            println!("initialize: {}", indices.is_some());
            let vao = VertexArray::default();
            vao.set_vertices(vertices, indices);
            let indices = indices.map(|s| s.to_vec());
            *self.0.0.lock().unwrap() = Some(MeshInner {
                vertices: vertices.to_vec(),
                indices,
                vao,
            });
            println!("Finished initializing.");
        }
    }

    pub fn set_vertices(&self, vertices: &[V]) where V: std::fmt::Debug {
        let mut me = self.0.0.lock().unwrap();
        let me: &mut MeshInner<V> = me.as_mut().unwrap();
        //println!("{:?}", vertices);
        me.vertices = vertices.to_vec();
        me.vao.set_vertices(&me.vertices, me.indices.as_deref());
    }

    pub fn draw(&self) {
        let mut me = self.0.0.lock().unwrap();
        let me: &mut MeshInner<V> = me.as_mut().unwrap();

        me.vao.bind();

        unsafe {
            if let Some(indices) = &me.indices {
                //me.vao.ebo();
                gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, null());
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
        }
    }
}

impl<V: Vertex> Default for Mesh<V> {
    fn default() -> Self {
        Self(Atomic::default())
    }
}