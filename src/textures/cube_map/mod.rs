use lazy::lazy_object::LazyObject;

use crate::atomic::Atomic;
use crate::textures::cube_map::cube_map_textures::CubeMapTextures;
use crate::textures::texture_settings::TextureSettings;
use crate::textures::texture_type::TextureType;

pub mod imp;
pub mod cube_map_textures;

pub struct CubeMapLazy {
    texture_settings: TextureSettings,
    texture_type: TextureType,
    textures: CubeMapTextures
}

pub struct CubeMapInner {
    ptr: u32,
    texture_settings: TextureSettings,
    texture_type: TextureType,
}

pub struct CubeMap(Atomic<LazyObject<CubeMapLazy, CubeMapInner>>);