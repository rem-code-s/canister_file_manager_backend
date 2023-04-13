use candid::{CandidType, Deserialize};
use serde::Serialize;

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
