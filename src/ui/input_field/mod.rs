use crate::bundled_mesh::BundledMesh;
use crate::model::mesh::Mesh;
use crate::program::Program;
use crate::ui::UI;
use crate::vertex::pos2_uv::Position2Uv;

static mut BUNDLED: Option<BundledMesh<Position2Uv>> = None;

const INPUT_FIELD_VS: &'static str = include_str!("../../../res/font/input_field_vs.glsl");
const INPUT_FIELD_FS: &'static str = include_str!("../../../res/font/input_field_fs.glsl");

pub mod text_field;

pub trait InputField<T> where Self: UI {
    fn value(&self) -> String;
    fn to_value(&self) -> Option<T>;
    fn caret(&self) -> usize;

    fn bundled() -> &'static BundledMesh<Position2Uv> where Self: Sized {
        unsafe {
            match &BUNDLED {
                None => {
                    let program = Program::new_from_src(INPUT_FIELD_VS, INPUT_FIELD_FS).unwrap();
                    let mesh = Mesh::default();
                    mesh.initialize(&[], None);
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