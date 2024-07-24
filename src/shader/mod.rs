use crate::graphics_error::GraphicsError;
use crate::graphics_error::shader_type::ShaderType;

pub mod vertex_shader;
pub mod fragment_shader;

pub trait Shader {
    const SHADER_TYPE: ShaderType;
    fn new_from_src<S: ToString>(shader: S) -> Result<Self, GraphicsError> where Self: Sized + Default {
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

    fn check_for_errors(shader: u32) -> Option<String> {
        let status = crate::ffi::get_shader_iv(shader, gl::COMPILE_STATUS);
        match status == 1 {
            true => None,
            false => Some(crate::ffi::get_shader_info_log(shader, 512))
        }
    }
}