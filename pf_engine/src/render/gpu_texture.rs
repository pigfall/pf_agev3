use crate::render::state::PipelineState;

#[derive(Debug)]
pub struct GPUTexture{
    state: *mut PipelineState,
    texture: glow::Texture,
}

#[derive(Copy, Clone)]
pub enum GpuTextureKind {
    Line {
        length: usize,
    },
    Rectangle {
        width: usize,
        height: usize,
    },
    Cube {
        width: usize,
        height: usize,
    },
    Volume {
        width: usize,
        height: usize,
        depth: usize,
    },
}
