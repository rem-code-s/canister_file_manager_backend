use ic_cdk::{api::time, caller};

use crate::{
    models::asset_models::{FileEntity, FileResponse, Id, Permission},
    store::{Store, STORE},
};

impl Store {
    pub fn delete_file(file_id: Id) -> Result<(), String> {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            Self::_delete_file(file_id, &mut store)
        })
    }

    pub fn _delete_file(file_id: Id, store: &mut Store) -> Result<(), String> {
        let file = store.files.get(&file_id);
        if let Some(_file) = file {
            if _file.is_protected {
                return Err("File is protected".to_string());
            }

            if _file.owner != Some(caller()) {
                return Err("File is not owned by you".to_string());
            }

            let chunk_ids = _file.chunks.clone();
            for chunk_id in chunk_ids {
                store.chunks.remove(&chunk_id);
            }
            store.files.remove(&file_id);
        }
        Ok(())
    }

    pub fn change_file_permission(file_id: Id, permission: Permission) -> Result<(), String> {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            let file = store.files.get_mut(&file_id);
            if let Some(_file) = file {
                if _file.is_protected {
                    return Err("Directory is protected".to_string());
                }

                if _file.owner != Some(caller()) {
                    return Err("Directory is not owned by you".to_string());
                }

                _file.permission = permission;
                _file.updated_at = time();
            }
            Ok(())
        })
    }

    pub fn add_chunks(chunks: Vec<(Id, Vec<u8>)>) {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            for (chunk_id, bytes) in chunks {
                // Only let the owner of the file upload the corresponding chunk
                if let Some(_file) = store.files.iter().find(|f| f.1.chunks.contains(&chunk_id)) {
                    store.chunks.insert(chunk_id, bytes);
                }
            }
        });
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
}
