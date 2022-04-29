
pub struct Renderer{
    //egl: Egl14,
}

impl Renderer{
    pub fn new()->Self{
        Self{
            //egl: Egl14::entry_load().unwrap(),

        }
    }
}


unsafe impl Send for Renderer{}
unsafe impl Sync for Renderer{}
