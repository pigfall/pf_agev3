use super::loader::AssetLoader;

pub struct DebugLoader{
    extensions: &[&str],
}

impl DebugLoader{
    pub fn new(extensions: &[&str])->Self{
        Self{
            extensions:extensions,
        }
    }
}

#[derive(Debug)]
pub struct DebugAsset{

}

impl AssetLoader for DebugLoader{
    fn load(&self,asset_path:&AssetPath)->Box<dyn Any>{
        return Box::new(DebugAsset{});
    }

    fn extensions(&self)->&[&str]{
        return self.extensions;
    }
}
