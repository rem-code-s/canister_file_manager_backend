use std::collections::HashMap;

use candid::{CandidType, Deserialize};
use ic_certified_map::{Hash, RbTree};
use serde::Serialize;

use crate::store::{Store, STORE};

use super::{asset_models::Id, directory_models::DirectoryEntity, file_models::FileEntity};

#[derive(Clone, Debug, Default, CandidType, Serialize, Deserialize)]
pub struct Metadata {
    pub cycles: u64,
    pub stable_memory: u64,
    pub heap_memory: u64,
    pub file_count: u64,
    pub directory_count: u64,
    pub files_combined_bytes: u64,
    pub version: String,
}

#[derive(Default, Clone)]
pub struct AssetHashes {
    pub tree: RbTree<String, Hash>,
}

pub type Files = HashMap<Id, FileEntity>;
pub type Directories = HashMap<Id, DirectoryEntity>;
pub type Chunks = HashMap<Id, Vec<u8>>;

impl From<&Store> for AssetHashes {
    fn from(store: &Store) -> Self {
        let mut asset_hashes = Self::default();

        for (_key, asset) in store.files.iter() {
            asset_hashes.insert(asset, store);
        }

        asset_hashes
    }
}

impl AssetHashes {
    pub(crate) fn insert(&mut self, file: &FileEntity, store: &Store) {
        let path = Store::get_file_path(file, &store);
        self.tree.insert(format!("/{}", path), file.hash);
    }
}
