use std::ptr::null;
use mathematics::linear_algebra::matrix::types::Mat4F32;
use mathematics::linear_algebra::vector::types::Vector2F32;
use crate::atomic::Atomic;
use crate::ui::input_field::InputField;
use crate::ui::input_field::text_field::TextField;
use crate::ui::UI;

impl TextField {

}

impl Default for TextField {
    fn default() -> Self {
        Self(Atomic::default())
    }
}

impl UI for TextField {
    fn draw(&self, projection: &Mat4F32) {
        let program_vao = Self::bundled();

        let position = self.position();
        let size = self.size();
        let vertices = Self::build_rect(position, size);

        program_vao.mesh.initialize(&vertices, Some(&[0, 1, 2, 0, 3, 2]));

        program_vao.program.bind();
        program_vao.program.bind_uniform("projection", *projection);

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
        }
    }

    fn size(&self) -> Vector2F32 {
        self.0.0.lock().unwrap().size
    }

    fn position(&self) -> Vector2F32 {
        self.0.0.lock().unwrap().position
    }
}

impl InputField<String> for TextField {
    fn value(&self) -> String {
        self.0.0.lock().unwrap().buffer.clone()
    }

    fn to_value(&self) -> Option<String> {
        Some(self.0.0.lock().unwrap().buffer.clone())
    }

    fn caret(&self) -> usize {
        self.0.0.lock().unwrap().caret
    }
}