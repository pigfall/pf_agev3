use bevy::prelude::{App,Plugin,CoreStage,SystemStage};
use super::asset_server::{AssetServer,free_unused_assets_system};
use super::asset_stage::{AssetStage};
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
        app.add_stage(AssetStage::UpdateAssets,SystemStage::parallel());
    }
}
