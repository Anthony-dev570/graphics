use crate::model::mesh::Mesh;
use crate::program::Program;
use crate::vertex::Vertex;

pub struct BundledMesh<V: Vertex> {
    pub program: Program,
    pub mesh: Mesh<V>
}