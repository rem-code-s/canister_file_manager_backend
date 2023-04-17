use candid::Principal;
use ic_cdk::{
    api::{call, time},
    caller,
};

use crate::{
    models::{
        asset_models::{Id, Permission},
        directory_models::{DirectoryEntity, DirectoryResponse},
        file_models::FileEntity,
    },
    store::{Store, STORE},
};

impl Store {
    pub fn delete_directory(directory_id: u64) -> Result<(), String> {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            Self::_delete_directory(directory_id, &mut store)
        })
    }

    // This method happens recursively
    pub fn _delete_directory(directory_id: u64, store: &mut Store) -> Result<(), String> {
        let directories = store.directories.clone();
        let files = store.files.clone();
        let directory = store.directories.get_mut(&directory_id);
        match Self::check_directory_state(directory) {
            Err(err) => Err(err),
            Ok(_directory) => {
                let children: Vec<&DirectoryEntity> = directories
                    .values()
                    .filter(|child_dir| child_dir.parent_id == Some(_directory.id))
                    .collect();

                let files: Vec<&FileEntity> = files
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
                Ok(())
            }
        }
    }

    pub fn change_directory_name(
        directory_id: Id,
        name: String,
    ) -> Result<DirectoryResponse, String> {
        STORE.with(|store| {
            let invalid_chars = ["/", "*", "\\", ":", "?", "\"", "<", ">", "'"];

            if invalid_chars.iter().any(|&c| name.contains(c)) {
                return Err("Invalid directory name".to_string());
            }

            let mut store = store.borrow_mut();
            let directories = store.directories.clone();

            if directories.iter().any(|(_id, directory)| {
                directory.parent_id == Some(directory_id) && directory.name == name
            }) {
                return Err("Directory with same name already exists".to_string());
            }

            let directory = store.directories.get_mut(&directory_id);

            match Self::check_directory_state(directory) {
                Err(err) => Err(err),
                Ok(_directory) => {
                    _directory.name = name;
                    _directory.updated_at = time();

                    Ok(Self::map_directory_entity_to_directory_response(
                        _directory.clone(),
                        &store,
                    ))
                }
            }
        })
    }

    pub fn change_directory_permission(
        directory_id: Id,
        permission: Permission,
    ) -> Result<DirectoryResponse, String> {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            let directory = store.directories.get_mut(&directory_id);
            match Self::check_directory_state(directory) {
                Err(err) => Err(err),
                Ok(_directory) => {
                    _directory.permission = permission;
                    _directory.updated_at = time();
                    Ok(Self::map_directory_entity_to_directory_response(
                        _directory.clone(),
                        &store,
                    ))
                }
            }
        })
    }

    pub fn change_directory_parent(
        directory_id: Id,
        parent_id: Option<Id>,
    ) -> Result<DirectoryResponse, String> {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            let directories = store.directories.clone();
            let directory = store.directories.get_mut(&directory_id);
            match Self::check_directory_state(directory) {
                Err(err) => Err(err),
                Ok(_directory) => {
                    if let Some(_parent_id) = parent_id {
                        if let Some(_parent_directory) = directories.get(&_parent_id) {
                            if _parent_directory.owner != Some(caller()) {
                                return Err("Parent directory is not owned by you".to_string());
                            }
                        } else {
                            return Err("Parent directory does not exist".to_string());
                        }
                    }

                    _directory.parent_id = parent_id;
                    _directory.updated_at = time();
                    Ok(Self::map_directory_entity_to_directory_response(
                        _directory.clone(),
                        &store,
                    ))
                }
            }
        })
    }

    // TODO: recursive method // change owner to whitelist? 🤔
    pub fn change_directory_owner(direcory_id: Id, owner: Principal) -> Result<(), String> {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            let directory = store.directories.get_mut(&direcory_id);
            match Self::check_directory_state(directory) {
                Err(err) => Err(err),
                Ok(_directory) => {
                    _directory.owner = Some(owner);
                    _directory.updated_at = time();
                    Ok(())
                }
            }
        })
    }

    pub fn find_directory(parent_id: Option<u64>, child_name: String) -> Option<DirectoryEntity> {
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

    pub fn create_directory(
        name: String,
        permission: Permission,
        parent_id: Option<Id>,
    ) -> Result<DirectoryEntity, String> {
        let invalid_chars = ["/", "*", "\\", ":", "?", "\"", "<", ">", "'"];

        if invalid_chars.iter().any(|&c| name.contains(c)) {
            return Err("Invalid directory name".to_string());
        }

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

    fn check_directory_state(
        directory: Option<&mut DirectoryEntity>,
    ) -> Result<&mut DirectoryEntity, String> {
        if let Some(_directory) = directory {
            if _directory.is_protected {
                return Err("Directory is protected".to_string());
            }

            if _directory.owner != Some(caller()) {
                return Err("Directory is not owned by you".to_string());
            }

            return Ok(_directory);
        } else {
            return Err("Directory not found".to_string());
        }
    }
}
