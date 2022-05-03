#[cfg(target_os="android")]
pub mod android_log;
#[cfg(target_os="android")]
pub use android_log::*;

#[cfg(target_os="windows")]
pub mod windows_log;
#[cfg(target_os="windows")]
pub use windows_log::*; 


