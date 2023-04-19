use candid::{CandidType, Deserialize, Principal};
use ic_certified_map::Hash;
use serde::Serialize;

use super::asset_models::{Id, Manifest, Permission};

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
    pub hash: Hash,
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
