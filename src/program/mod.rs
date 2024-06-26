use std::ffi::CString;
use std::sync::{Arc, Mutex};
use crate::atomic::Atomic;

use crate::atomic_macro;
use crate::graphics_error::GraphicsError;
use crate::shader::fragment_shader::FragmentShader;
use crate::shader::Shader;
use crate::shader::vertex_shader::VertexShader;
use crate::uniform::Uniform;

atomic_macro!(
    pub struct Program {
        id: u32
    }
);

impl Drop for ProgramInner {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(ProgramInner {
            id: 0,
        })))
    }
}

impl Program {
    pub fn id(&self) -> u32 {
        self.0.lock().unwrap().id
    }
    pub fn is_initialized(&self) -> bool {
        self.0.lock().unwrap().id != 0
    }

    fn initialize(&self) {
        unsafe {
            if self.is_initialized() {
                gl::DeleteProgram(self.id());
            }
            self.0.lock().unwrap().id = gl::CreateProgram();
        }
    }

    pub fn load_from_source<V: ToString, F: ToString>(
        &self,
        vertex_shader: V,
        fragment_shader: F,
    ) -> Option<GraphicsError> {
        self.initialize();
        unsafe {
            let id = self.id();

            match VertexShader::new_from_src(vertex_shader) {
                Ok(vertex) => {
                    match FragmentShader::new_from_src(fragment_shader) {
                        Ok(fragment) => {
                            gl::AttachShader(id, vertex.id());
                            gl::AttachShader(id, fragment.id());
                            gl::LinkProgram(id);
                        }
                        Err(e) => return Some(e)
                    }
                }
                Err(e) => return Some(e)
            }
            //check for errors
        }
        None
    }

    pub fn new_from_src<V: ToString, F: ToString>(
        vertex_shader: V,
        fragment_shader: F,
    ) -> Result<Self, GraphicsError> {
        let out = Self::default();
        if let Some(error) = out.load_from_source(vertex_shader, fragment_shader) {
            return Err(error);
        }
        Ok(out)
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id());
        }
    }

    pub fn bind_uniform<N: ToString, T, U: Uniform<T>>(&self, name: N, uniform: U) {
        unsafe {
            self.bind();
            let name = CString::new(name.to_string()).unwrap();
            let location = gl::GetUniformLocation(self.id(), name.as_ptr() as *const _ as *const _);
            uniform.bind_uniform(location);
        }
    }

    pub fn bind_atomic_uniform<N: ToString, T, U: Uniform<T>>(&self, name: N, uniform: Atomic<U>) {
        unsafe {
            self.bind();
            let uniform = uniform.0.lock().unwrap();
            let name = CString::new(name.to_string()).unwrap();
            let location = gl::GetUniformLocation(self.id(), name.as_ptr() as *const _ as *const _);
            uniform.bind_uniform(location);
        }
    }
}