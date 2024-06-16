use std::ffi::CString;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use gl::{CompileShader, CreateShader, ShaderSource};

use crate::atomic_macro;
use crate::graphics_error::GraphicsError;
use crate::graphics_error::shader_type::ShaderType;
use crate::shader::Shader;

atomic_macro!(
    pub struct FragmentShader {
        id: u32
    }
);

impl FragmentShader {}

impl Drop for FragmentShaderInner {
    fn drop(&mut self) {
        FragmentShader::on_drop(self.id)
    }
}

impl Shader for FragmentShader {
    const SHADER_TYPE: ShaderType = ShaderType::FragmentShader;

    fn load_from_src<S: ToString>(&self, source: S) -> Option<GraphicsError> {
        let source = CString::new(source.to_string()).unwrap();

        unsafe {
            let shader = CreateShader(gl::FRAGMENT_SHADER);
            ShaderSource(
                shader,
                1,
                &source.as_ptr() as *const _ as *const _,
                null(),
            );
            CompileShader(shader);
            *self.0.lock().unwrap() = FragmentShaderInner {
                id: shader,
            };

            //todo, do error checking
        }

        None
    }

    fn id(&self) -> u32 {
        self.0.lock().unwrap().id
    }
}

impl Default for FragmentShader {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(FragmentShaderInner { id: 0 })))
    }
}