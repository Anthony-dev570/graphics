use mathematics::linear_algebra::matrix::types::Mat4F32;
use mathematics::linear_algebra::vector::types::{Vector2, Vector2F32};

use crate::vertex::pos2_uv::Position2Uv;

pub mod input_field;

pub trait UI {
    fn draw(&self, projection: &Mat4F32);
    fn size(&self) -> Vector2F32;
    fn position(&self) -> Vector2F32;

    fn set_position(&self, position: Vector2F32);
    fn set_size(&self, size: Vector2F32);

    fn build_rect(position: Vector2F32, size: Vector2F32) -> [Position2Uv; 4] {
        let (x, y) = (position[0], position[1]);
        let (w, h) = (size[0], size[1]);
        [
            Position2Uv {
                position,
                uv: Default::default(),
            },
            Position2Uv {
                position: Vector2::new([x + w, y]),
                uv: Vector2::new([1_f32, 0_f32]),
            },
            Position2Uv {
                position: Vector2::new([x + w, y + h]),
                uv: Vector2::new([1_f32, 1_f32]),
            },
            Position2Uv {
                position: Vector2::new([x, y + h]),
                uv: Vector2::new([0_f32, 1_f32]),
            },
        ]
    }
}