use mathematics::color::Color;
use crate::graphics_pointer::GraphicsPointer;

use crate::objects::light::light_type::LightType;
use crate::uniform::Uniform;
use crate::uniform::uniform_struct::UniformStruct;

pub mod light_type;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    light_type: LightType,
    ambient: Color,
    diffuse: Color,
    specular: Color,
}

impl UniformStruct for Light {
    type Fields = (*const i32, *const Color, *const Color, *const Color);

    fn fields(&self) -> Self::Fields {
        (
            &self.light_type.light_id() as *const _,
            &self.ambient as *const _,
            &self.diffuse as *const _,
            &self.specular as *const _
        )
    }
}