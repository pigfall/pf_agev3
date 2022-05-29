#[cfg(target_os="android")]
pub mod android_fs;
#[cfg(target_os="android")]
pub use android_fs::*;

#[cfg(target_os="windows")]
pub mod windows_fs;
#[cfg(target_os="windows")]
pub use windows_fs::*;

pub mod error;
pub use error::Error;
