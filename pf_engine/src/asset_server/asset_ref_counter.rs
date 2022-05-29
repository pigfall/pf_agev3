use std::sync::mpsc::{Receiver,sync_channel,SyncSender};
use super::handle::{HandleId};
use std::collections::{HashMap};
pub struct AssetRefCounter{
    pub(crate) channel: RefChangeChannel,
    pub(crate) ref_counts: HashMap<HandleId,usize>,
}

impl Default for AssetRefCounter{
    fn default()->Self{
        let (sender,receiver) =sync_channel::<RefChange>(100);
        Self{
            channel: RefChangeChannel{
                sender,
                receiver,
            },
            ref_counts: Default::default(),
        }
    }
}

unsafe impl Send for AssetRefCounter{}
unsafe impl Sync for AssetRefCounter{}

pub struct RefChangeChannel{
    pub(crate) sender: SyncSender<RefChange>,
    pub(crate) receiver: Receiver<RefChange>,
}


pub enum RefChange{
    Increment(HandleId),
    Decrement(HandleId),
}
