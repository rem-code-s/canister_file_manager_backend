use ic_cdk::{api::time, caller};

use crate::{
    models::asset_models::{
        Asset, DirectoryEntity, DirectoryResponse, FileEntity, FileResponse, Id, NestedAssets,
        PostAsset,
    },
    store::{Store, STORE},
};

impl Store {
    pub fn add_assets(
        parent_id: Option<Id>,
        assets: Vec<NestedAssets>,
    ) -> Result<Vec<(FileResponse, String)>, (Vec<Asset>, String)> {
        STORE.with(|store| {
            let mut store = store.borrow_mut();
            // check if the files / directories are protected or owned by the caller
            if let Some(_parent_id) = parent_id {
                if let Some(_directory) = store.directories.get(&_parent_id) {
                    if _directory.is_protected {
                        return Err((vec![], format!("Directory {} is protected", _parent_id)));
                    }

                    if _directory.owner != Some(caller()) {
                        return Err((
                            vec![],
                            format!("Directory {} is not owned by you", _parent_id),
                        ));
                    }
                }
            }

            Self::add_assets_recursive(parent_id, assets, &mut store, false)
        })
    }

    // Add multiple files and directories to the store defined by the Asset type
    fn add_assets_recursive(
        parent_id: Option<Id>,
        assets: Vec<NestedAssets>,
        store: &mut Store,
        is_protected: bool,
    ) -> Result<Vec<(FileResponse, String)>, (Vec<Asset>, String)> {
        // Initialize an empty files vector and origin path on the user his file system to return
        let mut files: Vec<(FileResponse, String)> = vec![];
        let mut protected_assets: Vec<Asset> = vec![];

        for nested_asset in assets.clone() {
            match nested_asset.asset {
                PostAsset::File(post_file) => {
                    // If the file already exists, remove the existing file and its chunks
                    if let Some(existing_file) = store.files.clone().values().find(|_exisiting| {
                        _exisiting.parent_id == parent_id && _exisiting.name == post_file.name
                    }) {
                        match Self::_delete_file(existing_file.id, store) {
                            Ok(_) => {}
                            Err(_) => {
                                protected_assets.push(Asset::File(
                                    Self::map_file_entity_to_file_response(
                                        existing_file.clone(),
                                        store,
                                    ),
                                ));
                            }
                        }
                    }
                }
                PostAsset::Directory(post_directory) => {
                    if let Some(existing_directory) =
                        store.directories.clone().values().find(|_exisiting| {
                            _exisiting.parent_id == parent_id
                                && _exisiting.name == post_directory.name
                        })
                    {
                        match Self::_delete_directory(existing_directory.id, store) {
                            Ok(_) => {}
                            Err(_) => {
                                protected_assets.push(Asset::Directory(
                                    Self::map_directory_entity_to_directory_response(
                                        existing_directory.clone(),
                                        &store,
                                    ),
                                ));
                            }
                        }
                    }
                }
                PostAsset::None => {}
            }
        }

        if !protected_assets.is_empty() && !is_protected {
            return Err((
                protected_assets,
                "Some assets are protected or not owned by you".to_string(),
            ));
        }

        // Iterate over the assets
        for nested_asset in assets {
            match nested_asset.asset {
                // If the asset is a file
                PostAsset::File(post_file) => {
                    // get the file id
                    let file_id = store.file_id;

                    // Create the file entry from post_file
                    let mut file = FileEntity {
                        id: file_id,
                        name: post_file.name.clone(),
                        size: post_file.size,
                        mime_type: post_file.mime_type,
                        extension: post_file.extension,
                        permission: post_file.permission,
                        parent_id,
                        chunks: vec![],
                        metadata: post_file.metadata,
                        created_at: time(),
                        updated_at: time(),
                        is_protected,
                        owner: Some(caller()),
                    };

                    // Reserve the chunk ids for the file
                    for _ in 0..post_file.chunk_count {
                        let chunk_id = store.chunk_id;
                        store.chunks.insert(chunk_id, vec![]);
                        file.chunks.push(chunk_id);
                        store.chunk_id += 1;
                    }

                    // Insert the file into the store
                    store.files.insert(file_id, file.clone());

                    // Increment the file id
                    store.file_id += 1;

                    // Map the file entity to a file response and push it to the files vector
                    files.push((
                        Self::map_file_entity_to_file_response(file, &store),
                        post_file.origin_path,
                    ));
                }
                PostAsset::Directory(post_directory) => {
                    // Get the directory id
                    let directory_id = store.directory_id;

                    // Create the directory entry from post_directory
                    let directory = DirectoryEntity {
                        id: directory_id,
                        name: post_directory.name.clone(),
                        parent_id,
                        permission: post_directory.permission,
                        created_at: time(),
                        updated_at: time(),
                        is_protected,
                        owner: Some(caller()),
                    };

                    // Insert the directory into the store
                    store.directories.insert(directory_id, directory);

                    // Increment the directory id
                    store.directory_id += 1;

                    // Recursively add the nested assets by calling this method again
                    let files_from_directory = Self::add_assets_recursive(
                        Some(directory_id),
                        nested_asset.children,
                        store,
                        is_protected,
                    );

                    if let Ok(mut _files_from_directory) = files_from_directory {
                        files.append(&mut _files_from_directory);
                    }
                    // Append the files from the directory to the files vector
                }
                // If the asset type is None, do nothing (this should never happen)
                PostAsset::None => {}
            }
        }

        // Return the files vector
        Ok(files)
    }

    // Get all files and directories in a tree structure (parent -> children)
    pub fn get_assets_tree(parent_id: Option<Id>) -> Vec<Asset> {
        STORE.with(|store| {
            let store = store.borrow();
            // Get all directories filtered by parent_id (root is None)
            let mut directories: Vec<DirectoryResponse> = store
                .directories
                .values()
                // Map the entities to the correct response
                .map(|d| Self::map_directory_entity_to_directory_response(d.clone(), &store))
                .collect();

            // Get all child assets (files and directories) for the directory
            Self::get_assets_recursive(parent_id, &mut directories, &store)
        })
    }

    fn get_assets_recursive(
        parent_id: Option<Id>,
        directories: &mut Vec<DirectoryResponse>,
        store: &Store,
    ) -> Vec<Asset> {
        // Find all directories with the given parent_id
        let mut _directories: Vec<DirectoryResponse> = directories
            .iter()
            .filter(|d| d.parent_id == parent_id)
            .cloned()
            .collect();

        // Find all files with the given parent_id
        let _files: Vec<FileEntity> = STORE.with(|store| {
            store
                .borrow()
                .files
                .values()
                .filter(|f| f.parent_id == parent_id)
                .cloned()
                .collect()
        });

        // Recursively get children of each directory
        for child in &mut _directories {
            child.children = Self::get_assets_recursive(Some(child.id), directories, store)
        }

        // Convert directories to assets and add them to the assets vec
        let mut assets: Vec<Asset> = _directories
            .into_iter()
            .map(|a| Asset::Directory(a.clone()))
            .collect();

        // Convert files to assets and add them to the assets vec
        _files.iter().for_each(|a| {
            assets.push(Asset::File(Self::map_file_entity_to_file_response(
                a.clone(),
                &store,
            )))
        });

        assets
    }

    pub fn get_directory_child_assets(parent_id: Id, store: &Store) -> Vec<Asset> {
        let mut assets: Vec<Asset> = vec![];

        store
            .directories
            .values()
            .filter(|dir| Some(parent_id) == dir.parent_id)
            .for_each(|_dir| {
                assets.push(Asset::Directory(
                    Self::map_directory_entity_to_directory_response(_dir.clone(), &store),
                ))
            });

        store
            .files
            .values()
            .filter(|file| Some(parent_id) == file.parent_id)
            .for_each(|_file| {
                assets.push(Asset::File(Self::map_file_entity_to_file_response(
                    _file.clone(),
                    &store,
                )))
            });
        assets
    }
}
