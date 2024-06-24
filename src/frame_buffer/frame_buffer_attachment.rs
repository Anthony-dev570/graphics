#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum FrameBufferAttachment {
    ColorAttachment(u32),
    DepthAttachment,
    StencilAttachment,
    DepthStencilAttachment
}

impl FrameBufferAttachment {
    pub fn id(&self) -> u32 {
        match self {
            FrameBufferAttachment::ColorAttachment(attachment) => gl::COLOR_ATTACHMENT0 + *attachment,
            FrameBufferAttachment::DepthAttachment => gl::DEPTH_ATTACHMENT,
            FrameBufferAttachment::StencilAttachment => gl::STENCIL_ATTACHMENT,
            FrameBufferAttachment::DepthStencilAttachment => gl::DEPTH_STENCIL_ATTACHMENT
        }
    }
}