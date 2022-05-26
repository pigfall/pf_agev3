use super::handle::{HandleId,Handle};
use std::collections::{HashMap};
use bevy::reflect::TypeUuid;
use bevy::asset::Asset;

pub struct Assets<T:Asset>{
    assets: HashMap<HandleId,T>,
}

impl<T:Asset> Assets<T>{
    pub fn new()->Self{
        Self{
            assets:Default::default(),
        }
    }
    pub fn insert(&mut self,id: HandleId,asset: T){
        self.assets.insert(id,asset);
    }

    pub fn remove(&mut self,id: HandleId){
        self.assets.remove(&id);
    }

    pub fn get(&self,id: &HandleId)->Option<&T>{
        return self.assets.get(id);
    }

    pub fn get_mut(&mut self,id: &HandleId)->Option<&mut T>{
        return self.assets.get_mut(id);
    }

    pub fn get_asset(&self,handle: &Handle<T>)->Option<&T>{
        return self.get(&handle.id);
    }

    pub fn get_asset_mut(&mut self,handle: &Handle<T>)->Option<&mut T>{
        return self.get_mut(&handle.id);
    }

}
