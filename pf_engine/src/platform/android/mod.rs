use crate::{
    bevy::prelude::{App},
    log::{init_android_logger,info,error},
    systems::gles_render::{Renderer},
};

use ndk_sys::{
    ANativeActivity,
};

use std::{
    os::raw::c_void,
    ptr::NonNull,
    fs::File,
    io::{BufReader,BufRead},
    ffi::{CStr,CString},
    os::unix::{
        prelude::RawFd,
        io::FromRawFd,
    },
    thread,
};

mod ndk_callback;
use ndk_callback::*;

mod game_looper;
use game_looper::*;

mod global_game_looper;
use global_game_looper::*;


pub unsafe fn game_main(
    build_game: fn(app: &mut App),
    activity_raw_ptr: *mut c_void,
    saved_state: *mut c_void,
    saved_size: usize,
    ){
    init_android_logger("pf_engine");
    info!("⌛ register native activity callback");
    let mut activity_ptr = NonNull::new(activity_raw_ptr as *mut  ANativeActivity).expect("activity_raw_ptr is nil");

    // { register callback
    let callbacks = activity_ptr
        .as_mut()
        .callbacks
        .as_mut()
        .expect("activity callback is nil");
    callbacks.onNativeWindowCreated = Some(on_native_window_created);
    callbacks.onNativeWindowDestroyed = Some(on_native_window_destroyed);
    //callbacks.onWindowFocusChanged =Some(on_native_window_focus_changed);
    callbacks.onInputQueueCreated = Some(on_input_queue_created);
    callbacks.onInputQueueDestroyed = Some(on_input_queue_destroyed);
    // }
    
    // { the thread to handle stderr and stdout
    let mut logpipe: [RawFd; 2] = Default::default();
    libc::pipe(logpipe.as_mut_ptr());
    libc::dup2(logpipe[1], libc::STDOUT_FILENO);
    libc::dup2(logpipe[1], libc::STDERR_FILENO);
    thread::spawn(move || {
        let file = File::from_raw_fd(logpipe[0]);
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        loop {
            buffer.clear();
            if let Ok(len) = reader.read_line(&mut buffer) {
                if len == 0 {
                    break;
                } else if let Ok(msg) = CString::new(buffer.clone()) {
                    error!("{:?}", msg);
                    // android_logger(Level::Info, tag, &msg);
                }
            }
        }
    });

    //}
    
    game_looper = Box::into_raw(Box::new(GameLooper::new()));
    build_game(&mut game_looper.as_mut().unwrap().app);

    game_looper.as_mut().unwrap().app.add_plugin(Renderer{});


    thread::spawn(|| {
        (*game_looper).loop_run();
    });
}
