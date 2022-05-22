use bevy::utils::AHasher;
use std::hash::{Hash,Hasher};

#[derive(Debug, Hash, Clone)]
pub struct AssetPath{
    path: String,
}

impl AssetPath{
    pub fn get_id(self)->AssetPathId{
        AssetPathId::from(self)
    }
    pub fn path(&self)->String{
        self.path.clone()
    }
}



#[derive(
    Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd
)]
pub struct AssetPathId (u64);

impl From<AssetPath> for AssetPathId{
    fn from(asset_path: AssetPath) -> Self{
        let p = asset_path.path();
        let mut hasher = get_hasher();
        asset_path.hash(&mut hasher);
        Self(hasher.finish())
    }

}
/// this hasher provides consistent results across runs
pub(crate) fn get_hasher() -> AHasher {
    AHasher::new_with_keys(42, 23)
}
