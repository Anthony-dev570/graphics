use crate::textures::image_object::ImageObject;

#[derive(Clone)]
pub struct CubeMapTextures {
    pub positive_x: ImageObject,
    pub negative_x: ImageObject,
    pub positive_y: ImageObject,
    pub negative_y: ImageObject,
    pub positive_z: ImageObject,
    pub negative_z: ImageObject,
}

impl CubeMapTextures {
    pub fn images(&self) -> [&ImageObject; 6] {
        [
            &self.positive_x,
            &self.negative_x,
            &self.positive_y,
            &self.negative_y,
            &self.positive_z,
            &self.negative_z,
        ]
    }
}

impl Into<[ImageObject; 6]> for CubeMapTextures {
    fn into(self) -> [ImageObject; 6] {
        [
            self.positive_x,
            self.negative_x,
            self.positive_y,
            self.negative_y,
            self.positive_z,
            self.negative_z,
        ]
    }
}