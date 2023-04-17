use candid::{CandidType, Deserialize};
use serde::Serialize;

use super::{
    directory_models::{DirectoryResponse, PostDirectory},
    file_models::{FileResponse, PostFile},
};

pub type Id = u64;
pub type ChunkCount = u64;
pub type Path = Vec<String>;
pub type Manifest = Vec<Id>;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct NestedAssets {
    pub asset: PostAsset,
    pub children: Vec<NestedAssets>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Asset {
    File(FileResponse),
    Directory(DirectoryResponse),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum AssetWithId {
    File(Id),
    Directory(Id),
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum PostAsset {
    None,
    File(PostFile),
    Directory(PostDirectory),
}

impl Default for PostAsset {
    fn default() -> Self {
        PostAsset::None
    }
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub enum Permission {
    Public,       // public file
    Private,      // private file
    Origin(Path), // http origin to access the resource
}

impl Default for Permission {
    fn default() -> Self {
        Permission::Private
    }
}
