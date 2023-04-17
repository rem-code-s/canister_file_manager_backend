use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use super::asset_models::{Asset, Id, Permission};

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
