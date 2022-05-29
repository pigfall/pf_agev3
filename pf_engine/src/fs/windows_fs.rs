use std::path::{Path};
use std::io as stdio;

#[cfg(target_os="windows")]
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn read_file<P: AsRef<Path>>(p: P)->Result<Vec<u8>,stdio::Error>{
    todo!("");
}

