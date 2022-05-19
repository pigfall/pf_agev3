use bevy::prelude::App;
use std::ops::Index;
use std::collections::HashMap;
pub use bevy::utils::Uuid;
use std::any::{Any,TypeId};
use std::path::PathBuf;
use std::marker::PhantomData;
use std::sync::RwLock;
use std::sync::Arc;

pub mod assets;
use assets::Assets;
pub mod handle;
use handle::Handle;
pub mod loader;
use loader::AssetLoader;
pub mod path;

pub struct AssetServer{
    assets_registerd: HashMap<TypeId,bool>,
    loaders: RwLock<Vec<Arc<dyn AssetLoader>>>,
    extension_to_loaders: HashMap<String,usize>,
}

impl AssetServer {
    pub fn new()->Self{
        Self{
            assets_registerd:Default::default(),
            loaders:Default::default(),
            extension_to_loaders: Default::default(),
        }
    }

    pub fn load<T:'static>(&self, path: PathBuf)->Handle<T> {
        let ext = path.extension().ok_or_else(||{
            let p = path.to_str().unwrap();
            panic!("load asset from file path {p}, can not decide to use which loader")
        }).unwrap();

        let loader_index = self.extension_to_loaders.get(ext.to_str().unwrap()).unwrap();
        let loader = &self.loaders.read().unwrap()[0];
        let any = loader.load().downcast::<T>().unwrap();

        return Handle{
            id: Uuid::new_v4(),
            marker: PhantomData::default(),
        }
    }

    pub fn add_loader<T>(&mut self, loader: T)
    where
        T: AssetLoader,
    {
        let mut loaders = self.loaders.write().unwrap();
        for extension in loader.extensions(){
            if self.extension_to_loaders.contains_key(&extension.to_string()){
                panic!("extension {extension} has been loader registerd")
            }
        }

        let index = loaders.len();
        for ext in loader.extensions(){
            self.extension_to_loaders.insert(ext.to_string(),index);
        }
        loaders.push(Arc::new(loader))
    }


    pub fn register_asset<T:'static>(&mut self, app: &mut App){
        let assets = self.register_asset_helper::<T>();
    }

    fn register_asset_helper<T:'static>(&mut self)->Assets<T>{
        let type_id = TypeId::of::<T>();
        if self.assets_registerd.contains_key(&type_id){
            panic!("repeated register asset type {:?}",type_id);
        }
        self.assets_registerd.insert(type_id,true);
        return Assets::new();
    }
}
