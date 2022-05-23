use super::handle::{HandleId};
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

}
