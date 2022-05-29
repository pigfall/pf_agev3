use std::any::Any;
use crate::asset_server::loader::AssetLoader;
use crate::asset_server::asset_path::{AssetPath};
use super::texture::{TextureData,Texture};
use std::path::{PathBuf};
use rustsdk::io::{read_file};

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
        let bytes = read_file(asset_path).map_err(|e|format!("{:?} {:?}",e,asset_path)).unwrap();
        return Box::new(Texture{
            data:Some(TextureData::new(PathBuf::from(asset_path.path()),bytes)),
            gpu_texutre:None,
        });
    }
}
