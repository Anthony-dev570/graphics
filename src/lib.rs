pub mod vertex;
pub mod vertex_array;
pub mod atomic;


#[cfg(test)]
mod tests {
    use crate::vertex_array::VertexArray;

    #[test]
    fn it_works() {
        let a = VertexArray::default();

    }
}