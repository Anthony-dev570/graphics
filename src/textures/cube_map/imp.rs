use gl::{TEXTURE_CUBE_MAP, TEXTURE_CUBE_MAP_POSITIVE_X};
use lazy::lazy::Lazy;
use lazy::lazy_object::LazyObject;

use crate::atomic::Atomic;
use crate::ffi::{bind_cube_map, gen_texture};
use crate::textures::cube_map::{CubeMap, CubeMapInner, CubeMapLazy, CubeMapTextures};
use crate::textures::texture_color::TextureColor;
use crate::textures::texture_settings::TextureSettings;
use crate::textures::texture_type::TextureType;

impl CubeMap {
    pub fn new(
        texture_settings: Option<TextureSettings>,
        textures: CubeMapTextures,
    ) -> CubeMap {
        Self(Atomic::new(LazyObject::new(Lazy::Uninitialized(CubeMapLazy {
            texture_settings: texture_settings.unwrap_or(Self::default_texture_settings()),
            texture_type: TextureType::TextureCubeMap,
            textures,
        }), Box::new(|lazy| {
            let texture_settings = lazy.texture_settings.clone();
            let texture_id = gen_texture();
            bind_cube_map(texture_id);

            unsafe {
                for (id, face) in lazy.textures.images().iter().enumerate() {
                    let (width, height, pixels) = face.extract(texture_settings.texture_color.clone());

                    let internal_color = texture_settings.clone().texture_color as u32 as i32;

                    unsafe {
                        gl::TexImage2D(
                            TEXTURE_CUBE_MAP_POSITIVE_X + id as u32,
                            0,
                            internal_color,
                            width,
                            height,
                            0,
                            internal_color as u32,
                            gl::UNSIGNED_BYTE,
                            pixels.as_ptr() as *const _
                        );

                        for (k, v) in &texture_settings.parameters {
                            gl::TexParameteri(TEXTURE_CUBE_MAP, *k, *v);
                        }

                    }
                }
            }

            CubeMapInner {
                ptr: texture_id,
                texture_settings: lazy.texture_settings.clone(),
                texture_type: TextureType::TextureCubeMap,
            }
        }))))
    }

    pub fn initialize(&self) {
        self.0.0.lock().unwrap().initialize()
    }

    pub fn get(&self) -> u32 {
        self.0.0.lock().unwrap().get().ptr
    }

    pub fn default_texture_settings() -> TextureSettings {
        TextureSettings {
            parameters: maplit::hashmap! {
                gl::TEXTURE_MAG_FILTER => gl::LINEAR as i32,
                gl::TEXTURE_MIN_FILTER => gl::LINEAR as i32,
                gl::TEXTURE_WRAP_S => gl::CLAMP_TO_EDGE as i32,
                gl::TEXTURE_WRAP_T => gl::CLAMP_TO_EDGE as i32,
                gl::TEXTURE_WRAP_R => gl::CLAMP_TO_EDGE as i32,
            },
            texture_color: TextureColor::Rgb,
        }
    }
}