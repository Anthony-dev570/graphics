use crate::graphics_error::shader_type::ShaderType;

pub mod shader_type;

#[derive(Debug, Clone)]
pub enum GraphicsError {
    ShaderError {
        shader_type: ShaderType,
        error_text: String
    }
}