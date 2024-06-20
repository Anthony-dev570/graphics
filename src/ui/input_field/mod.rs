use crate::bundled_mesh::BundledMesh;
use crate::model::mesh::Mesh;
use crate::program::Program;
use crate::ui::UI;
use crate::vertex::pos_uv_vertex::PositionUvVertex;

static mut BUNDLED: Option<BundledMesh<PositionUvVertex>> = None;

const INPUT_FIELD_VS: &'static str = include_str!("../../../res/font/input_field_vs.glsl");
const INPUT_FIELD_FS: &'static str = include_str!("../../../res/font/input_field_fs.glsl");

pub mod text_field;

pub trait InputField<T> where Self: UI {
    fn value(&self) -> String;
    fn to_value(&self) -> Option<T>;
    fn caret(&self) -> usize;

    fn bundled() -> &'static BundledMesh<PositionUvVertex> where Self: Sized {
        unsafe {
            match &BUNDLED {
                None => {
                    let program = Program::new_from_src(INPUT_FIELD_VS, INPUT_FIELD_FS).unwrap();
                    let mesh = Mesh::default();
                    mesh.initialize(&[], Some(&[0, 1, 2, 0, 3, 2]));
                    BUNDLED = Some(BundledMesh {
                        program,
                        mesh,
                    });
                    Self::bundled()
                }
                Some(bundled) => {
                    bundled
                }
            }
        }
    }
}