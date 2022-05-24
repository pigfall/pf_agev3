use super::texture::{Texture};
use crate::asset_server::handle::{Handle};
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Material {
    texture: Option<Handle<Texture>>,
}
