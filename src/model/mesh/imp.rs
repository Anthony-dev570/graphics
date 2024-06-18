use crate::atomic::Atomic;
use crate::model::mesh::{Mesh, MeshInner};
use crate::vertex::Vertex;
use crate::vertex_array::VertexArray;

impl <V: Vertex + Clone> Mesh<V> {
    pub fn initialized(&self) -> bool {
        self.0.0.lock().unwrap().is_some()
    }
    pub fn initialize(
        &self,
        vertices: &[V],
        indices: Option<&[u32]>
    ) {
        if !self.initialized() {
            let vao = VertexArray::default();
            vao.set_vertices(vertices, indices);
            let indices =indices.map(|s| s.to_vec());
            *self.0.0.lock().unwrap() = Some(MeshInner {
                vertices: vertices.to_vec(),
                indices,
                vao,
            });
        }
    }
}

impl<V: Vertex> Default for Mesh<V> {
    fn default() -> Self {
        Self(Atomic::default())
    }
}