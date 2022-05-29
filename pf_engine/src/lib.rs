#[cfg(target_os="android")]
pub use bevy;

pub mod platform;
pub mod log;
pub mod events;
pub mod systems;
pub use rg3d_core as core;
pub mod utils;
pub mod render;
pub mod asset_server;
pub mod fs;
