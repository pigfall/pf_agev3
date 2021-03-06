use crate::systems::surface::surface::SurfaceData;
use crate::core::{
    algebra::{Matrix4},
};
use crate::render::state::PipelineState;

use super::geometry_buffer::GeometryBuffer;
use super::native_buffer::GeometryBufferKind;

use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Mesh {
    surface: SurfaceData,
    geometry_buffer: Option<GeometryBuffer>,
}

unsafe impl Send for Mesh{}
unsafe impl Sync for Mesh{}



impl Mesh {
    pub fn cube()->Self {
        return Mesh{
            surface: SurfaceData::make_cube(Matrix4::identity()),
            geometry_buffer:None,
        }
    }

    pub fn draw(&mut self,state: &mut PipelineState) {
        if self.geometry_buffer.is_none(){
            self.geometry_buffer = Some(GeometryBuffer::from_surface_data(&self.surface, GeometryBufferKind::StaticDraw,state));
        }
        self.geometry_buffer.as_mut().unwrap().bind(state).draw();
    }
}
