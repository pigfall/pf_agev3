use std::collections::HashMap;
use bevy::utils::Uuid;

pub struct Assets<T>{
    assets: HashMap<HandleId, T>,
}

impl<T> Assets<T>{
    pub fn new()->Self{
        Self{
            assets: HashMap::new(),
        }
    }

}

pub type HandleId = Uuid;
