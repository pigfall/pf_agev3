use bevy::{
    prelude::{Plugin,App,CoreStage,ResMut,Res,With,Without},
    ecs::{
        event::{EventReader,EventWriter},
        schedule::SystemStage,
        system::Query,
    },
};

use glow::HasContext;

use crate::{
    log::{info},
    events::define::{SystemEvents},
    render::gpu_program::GPUProgram,
    render::mesh::Mesh,
    render::{Material,Texture},
    asset_server::{Assets},
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
     mut query: Query<&mut Mesh,(Without<Material>)>,
     mut material_mesh_query: Query<(&mut Mesh,&mut Material)>,
     mut texture_assets: ResMut<Assets<Texture>>,
     mut renderer: ResMut<Renderer>,
    ) {
    for ev in system_events.iter() {
        match ev {
            SystemEvents::WindowCreate(window_ptr)=> {
                let surface = renderer.egl.entry_create_surface(*window_ptr).unwrap();
                renderer.egl.attach_surface_to_ctx(surface).unwrap();
                info!("✅ attached new surface to elgl ctx ");
                if renderer.gpu_program.is_none(){
                    renderer.gpu_program = Some(GPUProgram::standard(&mut renderer.state))
                }
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

    // { bind gpu program
    renderer.bind_gpu_program();
    // }

    unsafe {
        renderer.state.gl.clear_color(0.1,0.2,0.3,0.5);
        renderer.state.gl.clear(glow::COLOR_BUFFER_BIT);
    }
    
    // { draw meshes
    for mut mesh in query.iter_mut(){
        mesh.draw(&mut renderer.state);
    }
    // }
    //
    // { draw material mesh
    for (mut mesh,mut material) in material_mesh_query.iter_mut(){
        material.bind(&mut texture_assets,&mut renderer.state);
        mesh.draw(&mut renderer.state);
    }
    
    // }

    unsafe{
        renderer.egl.swap_buffers();
    };
}
