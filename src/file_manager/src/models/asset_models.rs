use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

pub type Id = u64;
pub type ChunkCount = u64;
pub type Path = Vec<String>;
pub type Manifest = Vec<Id>;

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct FileEntity {
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub mime_type: String,
    pub extension: String,
    pub permission: Permission,
    pub parent_id: Option<Id>,
    pub chunks: Manifest,
    pub metadata: Option<String>,
    pub is_protected: bool,
    pub owner: Option<Principal>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct PostFile {
    pub name: String,
    pub size: u64,
    pub mime_type: String,
    pub extension: String,
    pub permission: Permission,
    pub parent_id: Option<Id>,
    pub chunk_count: u64,
    pub metadata: Option<String>,
    pub origin_path: String,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct FileResponse {
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub mime_type: String,
    pub extension: String,
    pub permission: Permission,
    pub parent_id: Option<Id>,
    pub chunks: Manifest,
    pub path: String,
    pub metadata: Option<String>,
    pub is_protected: bool,
    pub owner: Option<Principal>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct DirectoryEntity {
    pub id: u64,
    pub name: String,
    pub parent_id: Option<Id>,
    pub permission: Permission,
    pub is_protected: bool,
    pub owner: Option<Principal>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct PostDirectory {
    pub name: String,
    pub parent_id: Option<Id>,
    pub permission: Permission,
    pub children: Vec<Asset>,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct DirectoryResponse {
    pub id: u64,
    pub name: String,
    pub parent_id: Option<Id>,
    pub children: Vec<Asset>, // only used when getting the directory
    pub permission: Permission,
    pub is_protected: bool,
    pub owner: Option<Principal>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct NestedDirectories {
    pub name: String,
    pub children: Vec<NestedDirectories>,
}

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
