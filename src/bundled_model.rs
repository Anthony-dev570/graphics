use crate::model::Model;
use crate::program::Program;
use crate::vertex::Vertex;

#[allow(dead_code)]
pub struct BundledModel<V: Vertex> {
    model: Model<V>,
    program: Program
}