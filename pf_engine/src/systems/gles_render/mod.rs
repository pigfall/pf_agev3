use bevy::{
    prelude::{Plugin,App,CoreStage,ResMut},
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

mod renderer;
use renderer::Renderer;

pub struct RendererPlugin {

}


impl Plugin for RendererPlugin {
    fn build(&self,app:&mut App){
        // { create egl
        let mut egl = pf_egl::Egl14::entry_load().unwrap();
        egl.entry_once_init().unwrap();
        // }

        // { load gl functions
        let gl_fns = unsafe {
            glow::Context::from_loader_function_with_version_parse(
                |version_str|{
                    // TODO
                    info!("gl version {:?}",version_str);
                    Ok(
                        glow::Version {
                            major: 3,
                            minor: 2,
                            is_embedded: true,
                            revision: None,
                            vendor_info: "tzz".to_string(),
                        }

                      )
                }
                ,
                |name|{
                    info!("⌛ Loading {:?}",name);
                    egl.instance.get_proc_address(name).
                        map_or(std::ptr::null(),|ptr|{
                            info!("✅  Loaded {:?} {:?}",name,ptr);
                            ptr as *const _
                        })
                }).map_err(
                    |e|{
                        info!("❌ {:?}",e);
                        e
                    }
                    ).unwrap()
        };
        // }
        
        app.insert_resource(Renderer::new(egl, gl_fns));
        app.add_stage_after(CoreStage::Update,"render",SystemStage::single_threaded());
        app.add_system_to_stage("render",render_frame);
    }
}



fn render_frame(
     mut system_events: EventReader<SystemEvents>,
     mut renderer: ResMut<Renderer>,
    ) {
    for ev in system_events.iter() {
        match ev {
            SystemEvents::WindowCreate(window_ptr)=> {
                let surface = renderer.egl.entry_create_surface(*window_ptr).unwrap();
                renderer.egl.attach_surface_to_ctx(surface).unwrap();
                info!("✅ attached new surface to elgl ctx ");
            },
            SystemEvents::WindowDestroy(_)=> {
                renderer.egl.destroy_cur_surface().unwrap();
                info!("😈  destroyed egl surface ");
            }
        }
    }
    if renderer.egl.ctx.as_ref().unwrap().surface.is_none(){
        return;
    }
    info!("renderering");
}
