pub mod vertex;
pub mod vertex_array;
pub mod atomic;
pub mod program;
pub mod shader;
pub mod graphics_error;
pub mod graphics_pointer;
pub mod uniform;
pub mod font;

#[cfg(test)]
mod tests {
    use crate::vertex_array::VertexArray;

    #[test]
    fn it_works() {
        let a = VertexArray::default();
    }
}