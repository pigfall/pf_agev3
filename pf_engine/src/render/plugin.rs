use bevy::prelude::{Plugin,App};
use crate::asset_server::asset_server::{AssetServer,register_asset,add_loader};
use crate::asset_server::assets::{Assets};
use super::texture_loader::TextureAssetLoader;
use super::texture::TextureData;
pub struct RendererAssetPlugin{}

impl Plugin for RendererAssetPlugin{
    fn build(&self,app: &mut App){
        //let mut asset_server = app.world.get_resource_mut::<AssetServer>().unwrap();
        register_asset::<TextureData>(app);
        add_loader(app,Box::new(TextureAssetLoader::default()));
    }
}
