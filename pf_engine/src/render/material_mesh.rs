use bevy::prelude::{Bundle};
use super::{Material,Mesh};

#[derive(Bundle)]
pub struct MaterialMeshBundle {
    pub mesh: Mesh,
    pub material: Material,
}
