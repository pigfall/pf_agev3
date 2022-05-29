use super::texture::{Texture};
use super::{GPUTexture,PipelineState,FrameworkError};
use crate::asset_server::handle::{Handle};
use crate::asset_server::assets::{Assets};
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Material {
    #[allow(dead_code)]
    texture: Option<Handle<Texture>>,
}

impl Material {
    #[allow(dead_code)]
    pub(crate) fn bind(&mut self,texture_assets: &mut Assets<Texture>,state: &mut PipelineState)->Result<(),FrameworkError>{
        let mut texture = match self.texture.as_mut().and_then(
                |texture_handle|{
                    texture_assets.get_asset_mut(&texture_handle)
                }
            ){
            Some(texture)=>texture,
            None=>return Ok(()),
        };

        let gpu_texture:&GPUTexture = match &texture.data{
            None=>return Ok(()),
            Some(data)=>{
                match &texture.gpu_texutre{
                    None=>{
                        let gpu_texture =GPUTexture::new(
                            state,
                            data.kind.into(),
                            data.pixel_kind.into(),
                            data.minification_filter.into(),
                            data.magnification_filter.into(),
                            data.mip_count.try_into().unwrap(),
                            Some(data.bytes.as_ref()),
                            )?;
                        texture.gpu_texutre = Some(gpu_texture);
                        texture.gpu_texutre.as_ref().unwrap()
                    }
                    Some(gpu_texture)=>gpu_texture,
                }

            },
        };

        gpu_texture.bind(state,0);
        Ok(())

    }

}
