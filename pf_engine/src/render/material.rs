use super::texture::{Texture};
use crate::asset_server::handle::{Handle};

pub struct Material {
    texture: Option<Handle<Texture>>,
}
