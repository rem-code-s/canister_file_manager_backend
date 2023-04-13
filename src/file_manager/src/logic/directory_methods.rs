use ic_cdk::{api::time, caller};

use crate::{
    models::asset_models::{DirectoryEntity, DirectoryResponse, FileEntity, Id, Permission},
    store::{Store, STORE},
};

impl Store {
    pub fn create_directory(
        name: String,
        permission: Permission,
        parent_id: Option<Id>,
    ) -> Result<DirectoryEntity, String> {
        STORE.with(|store| {
            let mut store = store.borrow_mut();

            // check if the files / directories are protected or owned by the caller
            if let Some(_parent_id) = parent_id {
                if let Some(_directory) = store.directories.get(&_parent_id) {
                    if _directory.is_protected {
                        return Err(format!("Parent directory {} is protected", _parent_id));
                    }

                    if _directory.owner != Some(caller()) {
                        return Err(format!(
                            "Parent directory {} is not owned by you",
                            _parent_id
                        ));
                    }
                }
            }

            let directory_id = store.directory_id;
            let directory = DirectoryEntity {
                id: directory_id,
                name,
                parent_id,
                permission,
                created_at: time(),
                updated_at: time(),
                is_protected: false,
                owner: Some(caller()),
            };
            store.directories.insert(directory_id, directory.clone());
            store.directory_id += 1;
            Ok(directory)
        })
    }

    pub fn delete_directory(directory_id: u64) -> Result<(), String> {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            Self::_delete_directory(directory_id, &mut store)
        })
    }

    // This method happens recursively
    pub fn _delete_directory(directory_id: u64, store: &mut Store) -> Result<(), String> {
        let directory = store.directories.get(&directory_id);
        let _store = store.clone();
        if let Some(_directory) = directory {
            if _directory.is_protected {
                return Err("Directory is protected".to_string());
            }

            if _directory.owner != Some(caller()) {
                return Err("Directory is not owned by you".to_string());
            }

            let children: Vec<&DirectoryEntity> = _store
                .directories
                .values()
                .filter(|child_dir| child_dir.parent_id == Some(_directory.id))
                .collect();

            let files: Vec<&FileEntity> = _store
                .files
                .values()
                .filter(|_file| _file.parent_id == Some(_directory.id))
                .collect();

            for child in children {
                // ignore the result of the recursive call
                let _ = Self::_delete_directory(child.id, store);
            }

            for child in files {
                // ignore the result of the recursive call
                let _ = Self::_delete_file(child.id, store);
            }

            store.directories.remove(&directory_id);
        }
        Ok(())
    }

    pub fn find_dir(parent_id: Option<u64>, child_name: String) -> Option<DirectoryEntity> {
        STORE.with(|store| {
            let store = store.borrow();
            store
                .directories
                .values()
                .find(|dir| {
                    parent_id == dir.parent_id && child_name == dir.name.replace(" ", "%20")
                })
                .cloned()
        })
    }

    pub fn map_directory_entity_to_directory_response(
        directory: DirectoryEntity,
        store: &Store,
    ) -> DirectoryResponse {
        DirectoryResponse {
            id: directory.id,
            name: directory.name,
            permission: directory.permission,
            parent_id: directory.parent_id,
            created_at: directory.created_at,
            updated_at: directory.updated_at,
            children: Self::get_directory_child_assets(directory.id, store),
            is_protected: directory.is_protected,
            owner: directory.owner,
        }
    }
}
