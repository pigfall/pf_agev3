use bevy::prelude::{App,Plugin};
use super::asset_server::{AssetServer};
use std::sync::mpsc::{Receiver,channel};
use std::sync::Arc;
pub struct AssetPlugin{

}

impl Plugin for AssetPlugin{
    fn build(&self,app: &mut App){
        app.insert_resource(AssetServer::default());
    }
}
