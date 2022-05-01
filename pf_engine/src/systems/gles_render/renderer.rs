use crate::render::state::PipelineState;

pub struct Renderer{
    pub(in crate) state: PipelineState, 
    pub(in crate) egl: pf_egl::Egl14,
}

impl Renderer{
    pub fn new(egl: pf_egl::Egl14, gl_fns: glow::Context)->Self{
        Self{
            state: PipelineState::new(gl_fns),
            egl: egl,
        }
    }
}


unsafe impl Send for Renderer{}
unsafe impl Sync for Renderer{}
