use std::any::Any;
pub trait  AssetLoader:Send + Sync + 'static {
    fn extensions(&self)->&'static[&'static str];
    fn load(&self)->Box<dyn Any>;
}
