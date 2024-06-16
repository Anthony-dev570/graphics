use std::mem::size_of;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use crate::atomic_macro;
use crate::vertex::Vertex;

atomic_macro!(
    pub struct VertexArray {
        vao: u32,
        vbo: u32,
        ebo: Option<u32>
    }
);

impl VertexArray {
    pub fn vao(&self) -> u32 {
        self.0.lock().unwrap().vao
    }

    pub fn vbo(&self) -> u32 {
        self.0.lock().unwrap().vbo
    }

    pub fn ebo(&self) -> Option<u32> {
        self.0.lock().unwrap().ebo
    }

    pub fn is_initialized(&self) -> bool {
        self.vao() != 0
    }

    fn initialize_ebo(&self) {
        if self.ebo().is_none() {
            unsafe {
                let mut ebo = 0;
                gl::GenBuffers(1, &mut ebo);
                gl::BindBuffer(
                    gl::ELEMENT_ARRAY_BUFFER,
                    ebo,
                );
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    0,
                    null(),
                    gl::DYNAMIC_DRAW,
                );
            }
        }
    }

    fn uninitialize_ebo(&self) {
        let mut me = self.0.lock().unwrap();
        if let Some(ebo) = me.ebo {
            unsafe { gl::DeleteBuffers(1, &ebo); }
            me.ebo = None;
        }
    }

    fn initialize(&self) {
        if !self.is_initialized() {
            println!("Initialize VAO");
            unsafe {
                let (mut vao, mut vbo) = (0, 0);

                gl::GenVertexArrays(1, &mut vao);
                gl::BindVertexArray(vao);

                gl::GenBuffers(1, &mut vbo);
                gl::BindBuffer(
                    gl::ARRAY_BUFFER,
                    vbo,
                );

                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    0,
                    null(),
                    gl::DYNAMIC_DRAW,
                );
                gl::BindVertexArray(0);

                let mut t = self.0.lock().unwrap();

                t.vao = vao;
                t.vbo = vbo;
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao());
        }
    }

    pub fn set_vertices<V: Vertex>(&self, vertices: &[V], indices: Option<&[u32]>) {
        self.initialize();
        self.bind();

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo());
            println!("{:?}", size_of::<V>());
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<V>()) as isize,
                &vertices.as_ptr() as *const _ as *const _,
                gl::DYNAMIC_DRAW,
            );

            V::load_attrib_pointers();

            if let Some(indices) = indices {
                self.initialize_ebo();
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo().unwrap());
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (indices.len() * size_of::<u32>()) as isize,
                    &indices.as_ptr() as *const _ as *const _,
                    gl::DYNAMIC_DRAW,
                );
            } else {
                self.uninitialize_ebo();
            }
        }
    }
}

impl Default for VertexArray {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(VertexArrayInner {
            vao: 0,
            vbo: 0,
            ebo: None,
        })))
    }
}