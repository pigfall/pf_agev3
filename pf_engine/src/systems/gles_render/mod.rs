use bevy::{
    prelude::{Plugin,App},
    ecs::event::{EventReader,EventWriter},
};

use crate::{
    log::{info},
    events::define::{SystemEvents},
};

use std::{
};

pub struct Renderer{

}

impl Plugin for Renderer {
    fn build(&self,app:&mut App){
        info!("adding gles renderer plugin");
        app.add_system(render_frame);
        //app.add_system(debug);
        app.add_system(writer);
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
