use crate::atomic::Atomic;
use crate::vertex::Vertex;
use crate::vertex_array::VertexArray;

pub mod imp;

#[derive(Clone)]
pub struct MeshInner<V: Vertex> {
    pub vertices: Vec<V>,
    pub indices: Option<Vec<u32>>,
    vao: VertexArray
}

#[derive(Clone)]
pub struct Mesh<V: Vertex>(Atomic<Option<MeshInner<V>>>);