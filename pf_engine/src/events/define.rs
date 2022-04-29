use std::os::raw::c_void;

#[derive(Copy, Clone,Debug)]
pub enum SystemEvents{
    WindowCreate(*mut c_void),
    WindowDestroy(*mut c_void),
}


unsafe impl Send for SystemEvents{}
unsafe impl Sync for SystemEvents{}

