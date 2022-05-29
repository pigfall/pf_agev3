use rustsdk::io;
use std::path::{Path};
use super::{Error};

pub fn read_file<P: AsRef<Path>>(p: P)->Result<Vec<u8>,Error>{
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.map_err(|e|Error::JNIError(format!("{:?}",e)))?;
    return io::read_file(p).map_err(|e|Error::IOError(e));
}
