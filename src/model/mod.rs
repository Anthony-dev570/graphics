use crate::atomic::Atomic;
use crate::model::mesh::Mesh;
use crate::vertex::Vertex;

pub mod mesh;
pub mod imp;

pub struct ModelInner<V: Vertex> {
    meshes: Vec<Mesh<V>>,
    name: Option<String>,
}

#[derive(Clone)]
pub struct Model<V: Vertex>(Atomic<ModelInner<V>>);