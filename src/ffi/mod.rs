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

pub fn gen_frame_buffer() -> u32 {
    unsafe {
        let mut fbo = 0;
        gl::GenFramebuffers(1, &mut fbo);
        fbo
    }
}

pub fn gen_render_buffer() -> u32 {
    unsafe {
        let mut rbo = 0;
        gl::GenRenderbuffers(1, &mut rbo);
        rbo
    }
}