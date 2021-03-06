use bevy::utils::AHasher;
use std::hash::{Hash,Hasher};
use std::path::{Path};

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
    pub fn extension(&self)->String{
        Path::new(&self.path).extension().unwrap().to_str().unwrap().to_string()
    }
}

impl From<&str> for AssetPath{
    fn from(s: &str)->Self{
        AssetPath{
            path: s.to_string()
        }
    }
}

impl Into<String> for &AssetPath{
    fn into(self)->String{
        return self.path.clone();
    }
}

impl AsRef<Path> for AssetPath{
    fn as_ref(&self)->&Path{
        return self.path.as_str().as_ref();
    }
}


#[derive(
    Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd
)]
pub struct AssetPathId (u64);

impl From<AssetPath> for AssetPathId{
    fn from(asset_path: AssetPath) -> Self{
        let mut hasher = get_hasher();
        asset_path.hash(&mut hasher);
        Self(hasher.finish())
    }

}
/// this hasher provides consistent results across runs
pub(crate) fn get_hasher() -> AHasher {
    AHasher::new_with_keys(42, 23)
}
