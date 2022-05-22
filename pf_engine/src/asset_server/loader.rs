use super::asset_path::{AssetPath};
use std::any::{Any};
pub trait AssetLoader:Send + Sync{
    fn load(&self,asset_path:&AssetPath) -> Box<dyn Any>;
    fn extensions(&self)->&[&str];
}
