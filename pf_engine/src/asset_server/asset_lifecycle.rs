use super::handle::{HandleId};
use std::sync::mpsc::{SyncSender,Receiver,sync_channel};
use std::sync::Mutex;

pub(crate) struct AssetLifecycle {
    pub(crate)sender: SyncSender<AssetLifecycleEvent>,
    pub(crate)receiver: Mutex<Receiver<AssetLifecycleEvent>>,
}


pub enum AssetLifecycleEvent {
    Free(HandleId),
}

impl AssetLifecycle {
    pub fn free_asset(&self,id: HandleId){
        self.sender.send(AssetLifecycleEvent::Free(id)).unwrap();
    }
}

impl Default for AssetLifecycle{
    fn default()->Self{
        let (sender,receiver) = sync_channel(100);
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}
