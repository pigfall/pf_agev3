//use rustsdk::io;
use std::path::{Path};
use super::{Error};
use crate::platform::android::GAME_LOOPER;
use std::ffi::CString;

pub fn read_file<P: AsRef<Path>>(p: P)->Result<Vec<u8>,Error>{
    unsafe{
        return Ok(GAME_LOOPER.as_ref().unwrap().native_activity.asset_manager().open(&CString::new(p.as_ref().to_str().unwrap()).unwrap()).unwrap().get_buffer().map_err(|e|Error::JNIError(format!("{:?}",e)))?.to_vec());
    }
}
