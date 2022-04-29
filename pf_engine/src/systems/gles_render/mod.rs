use bevy::{
    prelude::{Plugin,App,CoreStage},
    ecs::{
        event::{EventReader,EventWriter},
        schedule::SystemStage,
    },
};

use crate::{
    log::{info},
    events::define::{SystemEvents},
};

use std::{
};

pub struct Renderer {

}

impl Plugin for Renderer {
    fn build(&self,app:&mut App){
        info!("adding gles renderer plugin");
        app.add_stage_after(CoreStage::Update,"render",SystemStage::single_threaded());
        app.add_system_to_stage("render",render_frame);
    }
}

fn debug(){
    info!("runned");
}

fn writer(
     mut system_events: EventWriter<SystemEvents>,
    ){
}

fn render_frame(
     mut system_events: EventReader<SystemEvents>,
    ){
    for ev in system_events.iter() {
        match ev {
            SystemEvents::WindowCreate(window_ptr)=>{
                todo!("gles handle event of window_create");
            },
            SystemEvents::WindowDestroy(_)=>{
                todo!("gles handle event of window_desctroy");
            }
        }
    }
}
