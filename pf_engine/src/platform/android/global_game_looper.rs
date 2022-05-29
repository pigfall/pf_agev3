use super::game_looper::GameLooper;

// pub static mut game_looper: Option<GameLooper> = None;
 pub static mut GAME_LOOPER: *mut GameLooper = std::ptr::null_mut();
