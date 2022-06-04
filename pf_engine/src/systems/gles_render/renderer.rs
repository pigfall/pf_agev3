use crate::render::state::PipelineState;
use crate::render::gpu_program::GPUProgram;
use crate::render::gpu_program::GpuProgramBinding;
use crate::render::mesh::Mesh;
use crate::render::{Material};
use crate::render::{Texture,GPUTexture};
use crate::asset_server::{Assets};
use bevy::ecs::system::Query;
use crate::log::{info};

pub struct Renderer{
    pub(in crate) state: PipelineState, 
    pub(in crate) egl: pf_egl::Egl14,
    pub(in crate) gpu_program: Option<GPUProgram>,
}

impl Renderer {
    pub fn new(egl: pf_egl::Egl14, gl_fns: glow::Context)->Self{
        let state = PipelineState::new(gl_fns);
        //let gpu_program = GPUProgram::standard(&mut state);
        Self{
            state: state,
            egl: egl,
            gpu_program: None,
        }
    }

    pub(in crate) fn bind_gpu_program(&mut self)->Option<GpuProgramBinding>{
        self.gpu_program.as_ref().and_then(|gpu_program|{
            Some(gpu_program.bind(&mut self.state))
        })
    }

    pub(in crate) fn draw_material_mesh(
        &mut self,material_mesh_query: Query<(&mut Mesh,&mut Material)>,
        texture_assets: &mut Assets<Texture>,
        ){
        for (mesh,material) in material_mesh_query.iter() {
            let texture_handle = match &material.texture {
                Some(texture) =>texture,
                None=>continue,
            };
            info!("texture_handle {:?}",texture_handle);

            let texture =match texture_assets.get_asset_mut(&texture_handle){
                Some(texture)=>texture,
                None=>continue,
            };
            info!("texture {:?}",texture);

            let gpu_texture = texture.gpu_texture.get_or_insert_with(
                || {
                    let  texture_data= texture.data.as_ref().unwrap();
                    info!("texture_data {:?}",texture_data);
                    GPUTexture::new(
                         &mut self.state,
                         texture_data.kind.into(),
                         texture_data.pixel_kind.into(),
                         texture_data.minification_filter.into(),
                         texture_data.magnification_filter.into(),
                         texture_data.mip_count.try_into().unwrap(),
                         Some(texture_data.bytes.as_ref()),
                        ).unwrap()
                }
                );
            info!("gpu_texture {:?}",gpu_texture);

            // { set texture
            self.state.uniform_1_i32(sampler_location,texture_unit_0);
            self.active_texture_unit(texture_unit_0);
            self.state.bind_texutre(texture_target_2d,gpu_texture.texture_buffer_object);

            
            // }
        }

    }
}


unsafe impl Send for Renderer{}
unsafe impl Sync for Renderer{}
