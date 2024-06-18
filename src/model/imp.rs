use crate::model::mesh::Mesh;
use crate::model::Model;
use crate::vertex::Vertex;

impl <V: Vertex> Model<V> {
    pub fn push_mesh(&self, mesh: Mesh<V>) {
        self.0.0.lock().unwrap().meshes.push(mesh);
    }
}