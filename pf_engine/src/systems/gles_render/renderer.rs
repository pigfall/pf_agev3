use crate::render::state::PipelineState;
use crate::render::gpu_program::GPUProgram;

pub struct Renderer{
    pub(in crate) state: PipelineState, 
    pub(in crate) egl: pf_egl::Egl14,
    //pub(in crate) gpu_program: GPUProgram,
}

impl Renderer {
    pub fn new(egl: pf_egl::Egl14, gl_fns: glow::Context)->Self{
        let mut state = PipelineState::new(gl_fns);
        //let gpu_program = GPUProgram::standard(&mut state);
        Self{
            state: state,
            egl: egl,
            //gpu_program: gpu_program,
        }
    }

    pub(in crate) fn bind_gpu_program(&mut self){
        // self.gpu_program.bind(&mut self.state);
    }
}


unsafe impl Send for Renderer{}
unsafe impl Sync for Renderer{}
