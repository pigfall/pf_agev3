use super::handle::{Handle,HandleId};
use super::asset_path::{AssetPath};
use super::asset_ref_counter::{AssetRefCounter};
use super::handle::{HandleUntyped};

pub struct AssetServer{
    asset_ref_counter : AssetRefCounter,
}

impl AssetServer{
    pub fn load<T,P: Into<AssetPath>>(&self, asset_path: P)->Handle<T>{
        let handle_id = HandleId::from(asset_path.into());
        return HandleUntyped::strong(handle_id,self.asset_ref_counter.channel.sender.clone()).typed();
    }
}
