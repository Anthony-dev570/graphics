pub fn gen_texture() -> u32 {
    unsafe {
        let mut texture = 0;
        gl::GenTextures(1, &mut texture);
        texture
    }
}

pub fn bind_cube_map(texture_id: u32) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id)
    }
}