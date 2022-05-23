use bevy::prelude::{Plugin,App};
use crate::asset_server::asset_server::{AssetServer};
use crate::asset_server::assets::{Assets};
use super::texture_loader::TextureAssetLoader;
use super::texture::TextureData;
pub struct RendererAssetPlugin{}

impl Plugin for RendererAssetPlugin{
    fn build(&self,app: &mut App){
        let mut asset_server = app.world.get_resource_mut::<AssetServer>().unwrap();
        asset_server.add_loader(Box::new(TextureAssetLoader::default()));
        app.insert_resource(Assets::<TextureData>::new());
    }
}
