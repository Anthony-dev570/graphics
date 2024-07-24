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

pub fn get_program_iv(program: u32, name: u32) -> i32 {
    unsafe {
        let mut success = 0;
        gl::GetProgramiv(program, name, &mut success);
        success
    }
}

pub fn get_program_info_log(program: u32, size: usize) -> String {
    unsafe {
        let mut log = vec![0; size];
        let mut len = 0;
        gl::GetProgramInfoLog(
            program,
            size as i32,
            &mut len,
            log.as_mut_ptr()
        );

        let log_prime_ptr = &log[..len as usize] as *const [i8] as *const [u8];
        let log_prime = &*log_prime_ptr;

        String::from_utf8_lossy(&log_prime).to_string()
    }
}

pub fn get_shader_iv(shader: u32, name: u32) -> i32 {
    unsafe {
        let mut success = 0;
        gl::GetShaderiv(shader, name, &mut success);
        success
    }
}

pub fn get_shader_info_log(shader: u32, size: usize) -> String {
    unsafe {
        let mut log = vec![0; size];
        let mut len = 0;
        gl::GetShaderInfoLog(
            shader,
            size as i32,
            &mut len,
            log.as_mut_ptr()
        );

        let log_prime_ptr = &log[..len as usize] as *const [i8] as *const [u8];
        let log_prime = &*log_prime_ptr;

        String::from_utf8_lossy(&log_prime).to_string()
    }
}