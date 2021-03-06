use std::marker::PhantomData;
use super::asset_path::{AssetPathId,AssetPath};
use std::sync::mpsc::{ SyncSender};
use super::asset_ref_counter::{RefChange};

#[derive(Debug)]
pub struct Handle<T>{
    pub(crate)id: HandleId,
    handle_type: HandleType,
    marker: PhantomData<T>,
}

impl<T> Clone for Handle<T>{
    fn clone(&self)->Self{
        match self.handle_type{
            HandleType::Weak=>Handle::weak(self.id),
            HandleType::Strong(ref sender)=>Handle::strong(self.id,sender.clone()),
        }
    }

}

impl<T> Handle<T>{
    fn strong(id: HandleId,sender: SyncSender<RefChange>)->Self{
        HandleUntyped::strong(id,sender).typed()
    }
    fn weak(id: HandleId)->Self{
        Self {
            id,
            handle_type: HandleType::Weak,
            marker:PhantomData,
        }
    }
}


#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
)]
pub struct HandleId (pub AssetPathId);

impl From<AssetPath> for HandleId{
    fn from(asset_path: AssetPath)->Self{
        Self(asset_path.get_id())
    }
}

pub struct HandleUntyped{
    id: HandleId,
    handle_type: HandleType
}

impl HandleUntyped{
    pub(crate) fn strong(handle_id:HandleId,sender: SyncSender<RefChange>)->Self{
        sender.send(RefChange::Increment(handle_id)).unwrap();
        Self{
            id:handle_id,
            handle_type:HandleType::Strong(sender),
        }
    }

    pub fn typed<T>(mut self)->Handle<T>{
        let handle_type = match &self.handle_type{
            HandleType::Weak => HandleType::Weak,
            HandleType::Strong(sender)=> HandleType::Strong(sender.clone()),
        };

        self.handle_type = HandleType::Weak;

        Handle{
            id:self.id,
            handle_type:handle_type,
            marker:PhantomData,
        }

    }
}

#[derive(Debug)]
pub enum HandleType{
    Weak,
    Strong(SyncSender<RefChange>)
}
