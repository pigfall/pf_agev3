use ndk_sys::{
ANativeActivity,ANativeWindow,AInputQueue,
};

use super::GAME_LOOPER;

#[allow(unused_variables)]
pub unsafe extern "C" fn on_native_window_created(activity_raw: *mut ANativeActivity,window_raw: *mut ANativeWindow){
    (*GAME_LOOPER).on_window_create(window_raw);
}

#[allow(unused_variables)]
pub unsafe extern "C" fn  on_native_window_destroyed(activity_raw: *mut ANativeActivity,window_raw: *mut ANativeWindow){
    (*GAME_LOOPER).on_window_destroy(window_raw);
}

#[allow(unused_variables)]
pub unsafe extern "C" fn on_input_queue_created(activity_raw: *mut ANativeActivity,queue:*mut AInputQueue){
}

#[allow(unused_variables)]
pub unsafe extern "C" fn on_input_queue_destroyed(activity_raw: *mut ANativeActivity,queue:*mut AInputQueue){
}

