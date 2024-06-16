pub mod pos_vertex;
pub mod pos_uv_vertex;
pub mod pos_uv_norm_vertex;

pub trait Vertex {
    fn load_attrib_pointers();
}