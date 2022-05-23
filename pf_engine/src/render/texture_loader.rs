use std::any::Any;
use crate::asset_server::loader::AssetLoader;
use crate::asset_server::asset_path::{AssetPath};
use super::texture::TextureData;
use std::path::{PathBuf};

pub struct TextureAssetLoader{

}

impl Default for TextureAssetLoader{
    fn default()->Self{
        Self{}
    }
}

impl AssetLoader for TextureAssetLoader{
    fn extensions(&self)->&[&str]{
        return &["texture"];
    }
    fn load(&self,asset_path: &AssetPath)->Box<dyn Any>{
        return Box::new(TextureData::new(PathBuf::from(asset_path.path()),Vec::new()));
    }
}
