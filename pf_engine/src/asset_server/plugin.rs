use bevy::prelude::{App,Plugin,CoreStage};
use super::asset_server::{AssetServer,free_unused_assets_system};
use std::sync::mpsc::{Receiver,channel};
use std::sync::Arc;
pub struct AssetPlugin{

}

impl Plugin for AssetPlugin{
    fn build(&self,app: &mut App){
        app.insert_resource(AssetServer::default());
        app.add_system_to_stage(
            CoreStage::PreUpdate,
            free_unused_assets_system,
        );
    }
}
