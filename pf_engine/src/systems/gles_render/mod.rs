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

pub struct RendererPlugin {

}

impl Plugin for RendererPlugin {
    fn build(&self,app:&mut App){
        info!("adding gles renderer plugin");
        app.add_stage_after(CoreStage::Update,"render",SystemStage::single_threaded());
        app.add_system_to_stage("render",render_frame);
    }
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
