use ndk_sys::{
    ANativeActivity,ANativeWindow,AInputQueue,
};

use crate::{
    events::define::{SystemEvents},
    log::{info,error},
};
use bevy::{
    prelude::{App},
    ecs::event::{Events},
};

use std::{
    collections::{VecDeque},
    sync::{Mutex,Condvar},
};

pub struct GameLooper{
    pub(in crate) app: App,
    system_events: VecDeque<SystemEvents>,
    lock: Mutex<bool>,
    cond: Condvar,
    updated: bool,
}


impl GameLooper {
    pub(in crate) fn loop_run(&mut self){
        loop{
            let wait_updated = self.pre_handle_system_events();
            self.app.update();
            if wait_updated {
                self.updated = true;
                self.cond.notify_all();
            }
        }
    }

    pub(in crate) fn new()->Self{
        let mut app = App::new();
        app.add_event::<SystemEvents>();
        Self{
            app: app,
            system_events: VecDeque::new(),
            lock: Mutex::new(false),
            cond: Condvar::new() ,
            updated: false,
        }
    }

    fn pre_handle_system_events(&mut self)->bool{
        let guard = self.lock.lock().unwrap();
        while !self.system_events.is_empty(){
            let msg = self.system_events.pop_front().expect("has checked not empty");
            match msg{
                SystemEvents::WindowCreate(wrapper) =>{
                    info!("rcv msg window created");
                    let mut system_events = self.app.world.resource_mut::<Events<SystemEvents>>();
                    system_events.send(msg);
                    //renderer.on_window_create(wrapper.window as _);
                },
                SystemEvents::WindowDestroy(_)=>{
                    info!("rcv msg window destroy");
                    let mut system_events = self.app.world.resource_mut::<Events<SystemEvents>>();
                    system_events.send(msg);
                    //renderer.on_window_destroy(std::ptr::null_mut());
                },
                //AndroidActivityEvent::InputQueueCreated(queue)=>{
                //    info!("rcv msg input queue created");
                //    //self.input_queue = queue.queue;
                //},
                //AndroidActivityEvent::onInputQueueDestroyed=>{
                //    info!("rcv msg input destroy");
                //    //self.input_queue = std::ptr::null_mut();
                //},
            };
            return true;
        }
        return false;
    }

    pub(in crate) fn on_window_create(&mut self, window_raw: *mut ANativeWindow){
        self.require_lock_to_notify_msg_and_wait_handled(SystemEvents::WindowCreate(window_raw as _));
    }

    pub(in crate) fn on_window_destroy(&mut self, window_raw: *mut ANativeWindow){
        self.require_lock_to_notify_msg_and_wait_handled(SystemEvents::WindowDestroy(window_raw as _));
    }

    pub(in crate) fn on_input_queue_created(&mut self, activity_raw: *mut ANativeActivity,queue:*mut AInputQueue){

    }

    pub(in crate) fn on_input_queue_destroyed(&mut self, activity_raw: *mut ANativeActivity,queue:*mut AInputQueue){

    }

    fn require_lock_to_notify_msg_and_wait_handled(&mut self,msg: SystemEvents){
        info!("send msg {:?}",msg);
        let mut guard = self.lock.lock().unwrap();
        self.updated = false;
        self.system_events.push_back(msg);
        while !self.updated{
            info!("wating updated");
            guard =self.cond.wait(guard).unwrap();
        };
        info!("updated, unlocked");
    }
}

