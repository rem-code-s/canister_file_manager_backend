use std::{cell::RefCell, collections::HashMap};

use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use crate::models::asset_models::{DirectoryEntity, FileEntity, Id};

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Store {
    // Incrementing id for files
    pub file_id: Id,
    // Datastore for file entities
    pub files: HashMap<Id, FileEntity>,

    // Incrementing id for directories
    pub directory_id: Id,

    // Datastore for directory entities
    pub directories: HashMap<Id, DirectoryEntity>,

    // Incrementing id for chunks
    pub chunk_id: Id,

    // Datastore for chunks referenced in files
    pub chunks: HashMap<Id, Vec<u8>>,

    // The principal of the owner
    pub owner: Principal,
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

            owner: Principal::anonymous(),
            version: String::from("0.0.1"),
        }
    }
}

thread_local! {
    pub static STORE: RefCell<Store> = RefCell::new(Store::default());
}
