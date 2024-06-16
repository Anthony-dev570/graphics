use crate::graphics_error::GraphicsError;
use crate::graphics_error::shader_type::ShaderType;

pub mod vertex_shader;
pub mod fragment_shader;

pub trait Shader {
    const SHADER_TYPE: ShaderType;
    fn new<S: ToString>(shader: S) -> Result<Self, GraphicsError> where Self: Sized + Default {
        let out = Self::default();
        if let Some(error) = out.load_from_src(shader) {
            return Err(error);
        }
        Ok(out)
    }

    fn load_from_src<S: ToString>(&self, source: S) -> Option<GraphicsError>;
    fn id(&self) -> u32;
    fn on_drop(id: u32) {
        unsafe {
            gl::DeleteShader(id);
        }
    }
}