use super::handle::{HandleId};
use std::collections::{HashMap};

pub struct Assets<T>{
    assets: HashMap<HandleId,T>,
}

impl<T> Assets<T>{
    pub fn new()->Self{
        Self{
            assets:Default::default(),
        }
    }
    pub fn insert(&mut self,id: HandleId,asset: T){
        self.assets.insert(id,asset);
    }

}
