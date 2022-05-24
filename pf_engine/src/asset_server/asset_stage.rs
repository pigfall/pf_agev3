use bevy::ecs::schedule::StageLabel;

#[derive(StageLabel,Eq,PartialEq,Debug,Hash,Clone)]
pub(crate) enum AssetStage{
    UpdateAssets,
}
