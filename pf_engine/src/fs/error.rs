use std::io;

#[derive(Debug)]
pub enum Error{
    IOError(io::Error),
    #[cfg(target_os="android")]
    JNIError(String),
}
