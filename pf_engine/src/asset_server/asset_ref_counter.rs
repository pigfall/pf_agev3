use crossbeam_channel::{Receiver, Sender};
use super::handle::{HandleId};
pub struct AssetRefCounter{
    pub(crate) channel: RefChangeChannel
}

pub struct RefChangeChannel{
    pub(crate) sender: Sender<RefChange>,
    pub(crate) receiver: Receiver<RefChange>,
}

pub enum RefChange{
    Increment(HandleId),
    Decrement(HandleId),
}
