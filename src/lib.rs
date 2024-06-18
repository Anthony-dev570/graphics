pub mod vertex;
pub mod vertex_array;
pub mod atomic;
pub mod program;
pub mod shader;
pub mod graphics_error;
pub mod graphics_pointer;
pub mod uniform;
pub mod font;
pub mod texture;
pub mod a_or_b;
pub mod model;

#[cfg(test)]
mod tests {
    use crate::vertex_array::VertexArray;

    #[test]
    fn it_works() {
        let _a = VertexArray::default();
    }
}