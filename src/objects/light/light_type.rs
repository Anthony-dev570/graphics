use mathematics::linear_algebra::vector::types::Vector3F32;

#[derive(Debug, Clone, Copy)]
pub enum LightType {
    Directional {
        direction: Vector3F32,
    },
    Point {
        position: Vector3F32,

        constant: f32,
        linear: f32,
        quadratic: f32
    },
    Spotlight {
        position: Vector3F32,
        direction: Vector3F32,

        cut_off: f32,
    }
}

impl LightType {
    pub fn light_id(&self) -> i32 {
        match self {
            LightType::Directional { .. } => 0,
            LightType::Point { .. } => 1,
            LightType::Spotlight { .. } => 2,
        }
    }
}