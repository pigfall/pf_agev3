use super::handle::{HandleId};
use std::sync::mpsc::{SyncSender,Receiver};
use std::sync::Mutex;

pub(crate) struct AssetLifecycle {
    sender: SyncSender<AssetLifecycleEvent>,
    receiver: Mutex<Receiver<AssetLifecycleEvent>>,
}

pub enum AssetLifecycleEvent {
    Free(HandleId),
}

impl AssetLifecycle {
    pub fn free_asset(&self,id: HandleId){
        self.sender.send(AssetLifecycleEvent::Free(id)).unwrap();
    }
}
