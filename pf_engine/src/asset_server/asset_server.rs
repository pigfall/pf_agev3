use super::handle::{Handle,HandleId};
use super::asset_path::{AssetPath};
use super::asset_ref_counter::{AssetRefCounter};
use super::handle::{HandleUntyped};
use super::loader::{AssetLoader};
use super::assets::{Assets};
use bevy::prelude::{App};

pub struct AssetServer{
    asset_ref_counter : AssetRefCounter,
    loaders: Vec<Box<dyn AssetLoader>>,
}

impl Default for AssetServer{
    fn default() ->Self{
        Self{
            asset_ref_counter: Default::default(),
            loaders:Default::default(),
        }

    }
}


impl AssetServer{
    pub fn load<T:'static,P: Into<AssetPath>>(&self, path: P,assets: &mut Assets<T>)->Handle<T>{
        let asset_path = path.into();
        let handle_id = HandleId::from(asset_path.clone());
        let extension = asset_path.extension();
        let loader = self.loaders.iter().find(|loader| loader.extensions().iter().find(|&&ext| ext==extension ).is_some()).unwrap();
        let asset_any = loader.load(&asset_path);
        let asset = asset_any.downcast::<T>().unwrap();
        assets.insert(handle_id,*asset);
        return HandleUntyped::strong(handle_id,self.asset_ref_counter.channel.sender.clone()).typed();
    }

    pub fn add_loader(&mut self,loader: Box<dyn AssetLoader>){
        self.loaders.push(loader);
    }

    pub fn register_asset<T:'static+Send+Sync>(&mut self,app: &mut App){
        app.insert_resource(Assets::<T>::new());
    }
}
