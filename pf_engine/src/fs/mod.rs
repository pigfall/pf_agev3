#[cfg(target_os="android")]
pub mod android_fs;

#[cfg(target_os="android")]
pub use android_fs::*;


pub mod fs {
    #[cfg(target_os="android")]
    pub use super::android_fs::*;
}
