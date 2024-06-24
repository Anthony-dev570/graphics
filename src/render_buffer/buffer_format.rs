#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum BufferFormat {
    Depth24Stencil8 = gl::DEPTH24_STENCIL8
}