use super::texture::{Texture};
//use super::{GPUTexture,PipelineState,FrameworkError};
use crate::asset_server::handle::{Handle};
//use glow::HasContext;
//use crate::asset_server::assets::{Assets};
use bevy::ecs::component::Component;
//use crate::render::gpu_program::{GpuProgramBinding,GPUProgram};

#[derive(Component)]
pub struct Material {
    #[allow(dead_code)]
    pub texture: Option<Handle<Texture>>,
}

impl Material {

}
