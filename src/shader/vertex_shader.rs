use std::ffi::CString;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use gl::{CompileShader, CreateShader, ShaderSource};

use crate::atomic_macro;
use crate::graphics_error::GraphicsError;
use crate::graphics_error::shader_type::ShaderType;
use crate::shader::Shader;

atomic_macro!(
    pub struct VertexShader {
        id: u32
    }
);

impl Drop for VertexShaderInner {
    fn drop(&mut self) {
        VertexShader::on_drop(self.id)
    }
}

impl Shader for VertexShader {
    const SHADER_TYPE: ShaderType = ShaderType::VertexShader;

    fn load_from_src<S: ToString>(&self, source: S) -> Option<GraphicsError> {
        let source = CString::new(source.to_string()).unwrap();

        unsafe {
            let shader = CreateShader(gl::VERTEX_SHADER);
            ShaderSource(
                shader,
                1,
                &source.as_ptr() as *const _ as *const _,
                null()
            );
            CompileShader(shader);
            *self.0.lock().unwrap() = VertexShaderInner {
                id: shader,
            };

            if let Some(error) = Self::check_for_errors(shader) {
                return Some(GraphicsError::ShaderError {
                    shader_type: ShaderType::VertexShader,
                    error_text: error
                });
            }

            //todo, do error checking
        }

        None
    }

    fn id(&self) -> u32 {
        self.0.lock().unwrap().id
    }
}

impl Default for VertexShader {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(VertexShaderInner { id: 0 })))
    }
}