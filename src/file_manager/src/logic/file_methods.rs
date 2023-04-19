use candid::Principal;
use ic_cdk::{api::time, caller};
use sha2::{Digest, Sha256};

use crate::{
    models::{
        asset_models::{Id, Permission},
        file_models::{FileEntity, FileResponse},
    },
    store::{Store, ASSET_HASHES, STORE},
};

impl Store {
    pub fn delete_file(file_id: Id) -> Result<(), String> {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            Self::_delete_file(file_id, &mut store)
        })
    }

    pub fn _delete_file(file_id: Id, store: &mut Store) -> Result<(), String> {
        let file = store.files.get_mut(&file_id);
        match Self::check_file_state(file) {
            Err(err) => Err(err),
            Ok(_file) => {
                let chunk_ids = _file.chunks.clone();
                for chunk_id in chunk_ids {
                    store.chunks.remove(&chunk_id);
                }
                store.files.remove(&file_id);
                Ok(())
            }
        }
    }

    pub fn change_file_name(file_id: Id, name: String) -> Result<FileResponse, String> {
        let invalid_chars = ["/", "*", "\\", ":", "?", "\"", "<", ">", "'"];

        if invalid_chars.iter().any(|&c| name.contains(c)) {
            return Err("Invalid directory name".to_string());
        }

        STORE.with(|store| {
            let mut store = store.borrow_mut();
            let files = store.files.clone();

            if files
                .iter()
                .any(|(_id, _file)| _file.parent_id == Some(file_id) && _file.name == name)
            {
                return Err("File with same name already exists".to_string());
            }

            let file = store.files.get_mut(&file_id);
            match Self::check_file_state(file) {
                Err(err) => Err(err),
                Ok(_file) => {
                    _file.name = name;
                    _file.updated_at = time();
                    Ok(Self::map_file_entity_to_file_response(
                        _file.clone(),
                        &store,
                    ))
                }
            }
        })
    }

    pub fn change_file_permission(
        file_id: Id,
        permission: Permission,
    ) -> Result<FileResponse, String> {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            let file = store.files.get_mut(&file_id);
            match Self::check_file_state(file) {
                Err(err) => Err(err),
                Ok(_file) => {
                    _file.permission = permission;
                    _file.updated_at = time();
                    Ok(Self::map_file_entity_to_file_response(
                        _file.clone(),
                        &store,
                    ))
                }
            }
        })
    }

    pub fn change_file_owner(file_id: Id, owner: Principal) -> Result<(), String> {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            let file = store.files.get_mut(&file_id);
            match Self::check_file_state(file) {
                Err(err) => Err(err),
                Ok(_file) => {
                    _file.owner = Some(owner);
                    _file.updated_at = time();
                    Ok(())
                }
            }
        })
    }

    pub fn change_file_parent(file_id: Id, parent_id: Option<Id>) -> Result<FileResponse, String> {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            let directories = store.directories.clone();
            let file = store.files.get_mut(&file_id);
            match Self::check_file_state(file) {
                Err(err) => Err(err),
                Ok(_file) => {
                    if let Some(_parent_id) = parent_id {
                        if let Some(_parent_directory) = directories.get(&_parent_id) {
                            if _parent_directory.owner != Some(caller()) {
                                return Err("Parent directory is not owned by you".to_string());
                            }
                        } else {
                            return Err("Parent directory does not exist".to_string());
                        }
                    }

                    _file.parent_id = parent_id;
                    _file.updated_at = time();
                    Ok(Self::map_file_entity_to_file_response(
                        _file.clone(),
                        &store,
                    ))
                }
            }
        })
    }

    pub fn find_file(parent_id: Option<u64>, path_section: String) -> Option<FileEntity> {
        STORE.with(|store| {
            let store = store.borrow();
            store
                .files
                .values()
                .find(|file| {
                    parent_id == file.parent_id && path_section == file.name.replace(" ", "%20")
                })
                .cloned()
        })
    }

    pub fn add_chunks(chunks: Vec<(Id, Vec<u8>)>) {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            for (chunk_id, bytes) in chunks {
                // Only let the owner of the file upload the corresponding chunk
                if let Some((_, _file)) = store
                    .files
                    .clone()
                    .iter()
                    .find(|f| f.1.chunks.contains(&chunk_id))
                {
                    store.chunks.insert(chunk_id, bytes);

                    // if all chunks are uploaded
                    if _file
                        .clone()
                        .chunks
                        .iter()
                        .all(|id| store.chunks.contains_key(id))
                    {
                        let mut hasher = Sha256::new();

                        for chunk_id in _file.chunks.iter() {
                            let chunk = store.chunks.get(chunk_id);
                            if let Some(chunk) = chunk {
                                hasher.update(chunk);
                            }
                        }
                        let hash = hasher.finalize().into();

                        let mut updated_file = _file.clone();
                        updated_file.hash = hash;

                        ASSET_HASHES.with(|hashes| {
                            let mut hashes = hashes.borrow_mut();
                            ic_cdk::println!("test");
                            hashes.insert(&updated_file, &store);
                            Store::update_certified_data(&hashes);
                        });
                        store.files.insert(_file.id, updated_file);
                    }
                }
            }
        });
    }

    pub fn map_file_entity_to_file_response(file: FileEntity, store: &Store) -> FileResponse {
        let _file = file.clone();
        FileResponse {
            id: _file.id,
            name: _file.name,
            size: _file.size,
            mime_type: _file.mime_type,
            extension: _file.extension,
            permission: _file.permission,
            parent_id: _file.parent_id,
            chunks: _file.chunks,
            path: Self::get_file_path(&file, &store),
            metadata: _file.metadata,
            created_at: _file.created_at,
            updated_at: _file.updated_at,
            is_protected: _file.is_protected,
            owner: _file.owner,
        }
    }

    fn check_file_state(file: Option<&mut FileEntity>) -> Result<&mut FileEntity, String> {
        if let Some(_file) = file {
            if _file.is_protected {
                return Err("File is protected".to_string());
            }

            if _file.owner != Some(caller()) {
                return Err("File is not owned by you".to_string());
            }

            return Ok(_file);
        } else {
            return Err("File not found".to_string());
        }
    }
}
