
pub struct Renderer{
    pub(in crate) egl: pf_egl::Egl14,
    gl_fns: glow::Context,
}

impl Renderer{
    pub fn new(egl: pf_egl::Egl14, gl_fns: glow::Context)->Self{
        Self{
            egl: egl,
            gl_fns: gl_fns,
        }
    }
}


unsafe impl Send for Renderer{}
unsafe impl Sync for Renderer{}
