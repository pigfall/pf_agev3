use super::handle::{Handle,HandleId};
use super::asset_path::{AssetPath,AssetPathId};
use super::asset_ref_counter::{AssetRefCounter,RefChange};
use super::handle::{HandleUntyped};
use super::loader::{AssetLoader};
use super::assets::{Assets};
use bevy::asset::{Asset};
use super::asset_lifecycle::{AssetLifecycle};
use bevy::prelude::{App,Res,ResMut};
use std::sync::mpsc::{TryRecvError};
use std::collections::{HashMap};
use bevy::utils::Uuid;

pub struct AssetServer {
    asset_ref_counter : AssetRefCounter,
    loaders: Vec<Box<dyn AssetLoader>>,
    asset_lifecycles: HashMap<Uuid,AssetLifecycle>,
    source_info: HashMap<AssetPathId,Uuid>,  // asset_path to type_uuid
}

impl Default for AssetServer {
    fn default() ->Self{
        Self{
            asset_ref_counter: Default::default(),
            loaders:Default::default(),
            asset_lifecycles: Default::default(),
            source_info: Default::default(),
        }
    }
}


impl AssetServer{
    pub fn load<T:Asset,P: Into<AssetPath>>(&mut self, path: P,assets: &mut Assets<T>)->Handle<T>{
        let asset_path = path.into();
        let handle_id = HandleId::from(asset_path.clone());
        let extension = asset_path.extension();
        let loader = self.loaders.iter().find(|loader| loader.extensions().iter().find(|&&ext| ext==extension ).is_some()).unwrap();
        let asset_any = loader.load(&asset_path);
        let asset = asset_any.downcast::<T>().unwrap();
        assets.insert(handle_id,*asset);
        let asset_path_id = AssetPathId::from(asset_path.clone());
        self.source_info.insert(asset_path_id,T::TYPE_UUID);
        return HandleUntyped::strong(handle_id,self.asset_ref_counter.channel.sender.clone()).typed();
    }

    pub fn add_loader(&mut self,loader: Box<dyn AssetLoader>){
        self.loaders.push(loader);
    }

    pub fn register_asset<T:Asset+Send+Sync>(&mut self,app: &mut App){
        app.insert_resource(Assets::<T>::new());
    }

    pub(crate) fn free_unused_assets_system(&mut self) {
        let receiver = &self.asset_ref_counter.channel.receiver;
        loop{
            let ref_change = match receiver.try_recv(){
                Ok(ref_change)=>ref_change,
                Err(TryRecvError::Empty)=>break,
                Err(TryRecvError::Disconnected)=>panic!("unreachable")
            };
            match ref_change{
                RefChange::Increment(id)=> {
                    *self.asset_ref_counter.ref_counts.entry(id).or_insert(0) +=1 ;
                },
                RefChange::Decrement(id)=> {
                    let entry = self.asset_ref_counter.ref_counts.entry(id).or_insert(0);
                    *entry -=1;
                    if *entry == 0 {
                        // remove asset
                        let type_uuid = self.source_info.get(&id.0).unwrap();
                        if let Some(asset_lifecycle) = self.asset_lifecycles.get(type_uuid){
                            asset_lifecycle.free_asset(id);
                        }
                    }
                },
            }
        }


    }
}

pub(crate) fn free_unused_assets_system(mut asset_server: ResMut<AssetServer>){
    asset_server.free_unused_assets_system();
}

pub fn register_asset<T:Asset>(app:&mut App){
        let mut asset_server = app.world.get_resource_mut::<AssetServer>().unwrap();
        app.insert_resource(Assets::<T>::new());
}

pub fn add_loader(app:&mut App,loader:Box<dyn AssetLoader>){
        let mut asset_server = app.world.get_resource_mut::<AssetServer>().unwrap();
        asset_server.add_loader(loader);

}
