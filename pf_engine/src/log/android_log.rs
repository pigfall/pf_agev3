pub fn init_android_logger(tag: &str) {
    android_logger::init_once(
        android_logger::Config::default().
        with_min_level(log::Level::Trace). // NOTE: must need min level
        with_tag(tag),
    );
}

pub use log::{info,error};
