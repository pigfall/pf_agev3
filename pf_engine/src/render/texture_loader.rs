use bevy::asset::{AssetLoader,LoadedAsset};
use bevy::asset::LoadContext;
use std::error::Error;
use bevy::utils::BoxedFuture;
use anyhow::Result;
use super::texture::Texture;

#[derive(Default)]
pub struct TextureLoader {

}

impl AssetLoader for TextureLoader{
    fn load<'a>(
        &self,
        bytes: &[u8],
        load_ctx: &'a mut LoadContext,
        )->BoxedFuture<'a,Result<()>>{
        Box::pin(async{
            load_ctx.set_default_asset(LoadedAsset::new(Texture::default()));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str]{
        return &["texture"]
    }
}
