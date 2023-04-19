use std::{cell::RefCell, collections::HashMap};

use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use crate::models::{
    asset_models::Id,
    misc_models::{AssetHashes, Chunks, Directories, Files},
};

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Store {
    // Incrementing id for files
    pub file_id: Id,
    // Datastore for file entities
    pub files: Files,

    // Incrementing id for directories
    pub directory_id: Id,

    // Datastore for directory entities
    pub directories: Directories,

    // Incrementing id for chunks
    pub chunk_id: Id,

    // Datastore for chunks referenced in files
    pub chunks: Chunks,

    // The principal of the owner
    pub whitelist: Vec<Principal>,
    pub version: String,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            file_id: Default::default(),
            files: Default::default(),

            chunk_id: Default::default(),
            chunks: Default::default(),

            directory_id: Default::default(),
            directories: Default::default(),

            whitelist: Default::default(),
            version: String::from("0.0.1"),
        }
    }
}

thread_local! {
    pub static STORE: RefCell<Store> = RefCell::new(Store::default());
    pub static ASSET_HASHES: RefCell<AssetHashes> = RefCell::default();
}
