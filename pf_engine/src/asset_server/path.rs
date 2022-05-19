use std::hash::Hasher;
use std::path::{Path,PathBuf};
use std::borrow::Cow;
use bevy::utils::AHasher;
use std::hash::Hash;

pub struct AssetPath<'a>{
    path:Cow<'a, Path>,
    label: Option<Cow<'a, str >>
}


impl<'a> AssetPath<'a> {
    pub fn new(path: PathBuf, label: Option<String>)-> Self {
        Self{
            path: Cow::Owned(path),
            label: label.map(Cow::Owned),
        }
    }

    pub fn get_id(&self) -> AssetPathId {
        AssetPathId::from(self)
    }

    pub fn path(&self)->&Path{
        return &self.path;
    }
}

impl<'a> From<&AssetPath<'a>> for AssetPathId {
    fn from(asset_path: &AssetPath)-> Self{
        Self (
            SourcePathId::from(asset_path.path())
    )
}
}

pub struct AssetPathId(SourcePathId);

pub struct SourcePathId(u64);

impl From<&Path> for SourcePathId{
    fn from(value: &Path)->Self {
        let mut hasher = get_hasher();
        value.hash(&mut hasher);
        Self(hasher.finish())
    }

}

pub(crate) fn get_hasher()->AHasher{
    AHasher::new_with_keys(42,23)
}
