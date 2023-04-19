use ic_cdk::{
    api::{data_certificate, set_certified_data},
    id,
};
use ic_certified_map::{labeled, labeled_hash, AsHashTree};
use serde::Serialize;
use serde_cbor::Serializer;

use crate::{
    helpers::ic_data_helper,
    models::{
        file_models::FileEntity,
        http_models::{
            AssetEncoding, HeaderField, HttpRequest, HttpResponse, PathEntry,
            StreamingCallbackToken, StreamingStrategy,
        },
        misc_models::{AssetHashes, Metadata},
    },
    store::{Store, ASSET_HASHES, STORE},
};

impl Store {
    // Serve files over http
    // TODO: Serve folders over http
    pub fn http_request(req: HttpRequest) -> HttpResponse {
        // Get the path and split it by '/' so we can get the path segments; ex: ['directories', 'directory', 'file.txt']
        let mut path: Vec<&str> = req.url.as_str().split('/').collect();
        path = path.iter().filter(|p| !p.is_empty()).cloned().collect();

        // Create a permission denied response
        let not_found = HttpResponse {
            status_code: 404,
            headers: vec![],
            body: vec![],
            streaming_strategy: None,
        };

        // Create a path entry to serve content by
        let mut asset_paths = vec![PathEntry {
            // Segments to match the path against
            match_path: vec!["directories".to_string()],
            response: HttpResponse {
                status_code: 200,
                headers: vec![],
                body: serde_json::to_string(&Self::get_assets_tree(None, None))
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
                streaming_strategy: None,
            },
        }];

        // Implementation so serve files by the correct path
        if let Some(mut file_path) = Self::get_file_by_path(&path) {
            file_path.match_path = file_path
                .match_path
                .iter()
                .map(|s| s.replace(" ", "%20"))
                .collect();
            ic_cdk::println!("Matched path: {:?}", file_path.match_path);
            asset_paths.push(file_path);
        }

        match req.method.as_str() {
            "GET" => {
                let response = asset_paths.iter().find(|a| a.match_path == path);
                match response.cloned() {
                    None => not_found,
                    Some(_response) => _response.response,
                }
            }
            _ => not_found,
        }
    }

    pub fn streaming_strategy(
        file_id: &u64,
        encoding: &AssetEncoding,
        headers: &[HeaderField],
    ) -> Option<StreamingStrategy> {
        let streaming_token: Option<StreamingCallbackToken> =
            Self::create_token(file_id, 0, encoding, headers);

        streaming_token.map(|streaming_token| StreamingStrategy::Callback {
            callback: candid::Func {
                method: "http_request_streaming_callback".to_string(),
                principal: id(),
            },
            token: streaming_token,
        })
    }

    pub fn create_token(
        file_id: &u64,
        chunk_index: usize,
        encoding: &AssetEncoding,
        headers: &[HeaderField],
    ) -> Option<StreamingCallbackToken> {
        if chunk_index + 1 >= encoding.content_chunks.len() {
            return None;
        }
        Some(StreamingCallbackToken {
            file_id: file_id.clone(),
            headers: headers.to_owned(),
            chunk_index: chunk_index + 1,
            hash: encoding.hash.clone(),
        })
    }

    // This can probably change to file.path (not implemented for directories yet)
    pub fn get_file_by_path(path: &Vec<&str>) -> Option<PathEntry> {
        let mut file: Option<FileEntity> = None;

        if path.len() == 0 {
            file = Self::find_file(None, "index.html".to_string());
        } else {
            match Self::find_directory(None, path[0].to_string()) {
                Some(_directory) => {
                    let mut directory = _directory;
                    for section in path.iter().skip(1) {
                        match Self::find_directory(Some(directory.id), section.to_string()) {
                            Some(_directory) => {
                                directory = _directory;
                            }
                            None => {
                                file =
                                    match Self::find_file(Some(directory.id), section.to_string()) {
                                        Some(_file) => Some(_file),
                                        None => Self::find_file(
                                            Some(directory.id),
                                            "index.html".to_string(),
                                        ),
                                    }
                            }
                        };
                    }
                }
                None => {
                    file = match Self::find_file(None, path[0].to_string()) {
                        Some(_file) => Some(_file),
                        None => Self::find_file(None, "index.html".to_string()),
                    }
                }
            }
        }

        match file {
            Some(_file) => STORE.with(|store| {
                let store = store.borrow();

                let mut headers = vec![
                    HeaderField("content-type".to_string(), _file.mime_type.to_string()),
                    HeaderField("accept-ranges".to_string(), "bytes".to_string()),
                    HeaderField("content-length".to_string(), _file.size.to_string()),
                ];

                let cert_header = ASSET_HASHES.with(|hashes| {
                    Self::build_asset_certificate_header(
                        &hashes.borrow(),
                        format!("/{}", _file.name),
                    )
                });

                if let Ok(_header) = cert_header {
                    headers.push(_header);
                }

                let encoding = AssetEncoding {
                    content_chunks: _file.chunks.clone(),
                    bytes_length: _file.size as u128,
                    hash: _file.hash.clone(),
                };

                let body = store.chunks.get(&_file.chunks[0]).unwrap().clone();

                Some(PathEntry {
                    match_path: path.iter().map(|p| p.to_string()).collect(),
                    response: HttpResponse {
                        status_code: 200,
                        headers: headers.clone(),
                        body,
                        streaming_strategy: Self::streaming_strategy(
                            &_file.id, &encoding, &headers,
                        ),
                    },
                })
            }),
            None => None,
        }
    }

    pub fn get_directory_path_recursive(parent_id: u64, store: &Store, path: &mut Vec<String>) {
        let directory = store.directories.get(&parent_id);
        if let Some(_directory) = directory {
            path.push(_directory.name.clone());
            if let Some(parent_id) = _directory.parent_id {
                Self::get_directory_path_recursive(parent_id, store, path);
            }
        }
    }

    pub fn get_metadata() -> Metadata {
        STORE.with(|store| {
            let store = store.borrow();
            let files_combined_bytes = store.files.iter().fold(0, |acc, (_, file)| acc + file.size);
            Metadata {
                file_count: store.files.len() as u64,
                directory_count: store.directories.len() as u64,
                cycles: ic_data_helper::get_cycles(),
                heap_memory: ic_data_helper::get_heap_memory_size(),
                stable_memory: ic_data_helper::get_stable_memory_size(),
                version: store.version.clone(),
                files_combined_bytes,
            }
        })
    }

    pub fn get_file_path(file: &FileEntity, store: &Store) -> String {
        let mut path: Vec<String> = vec![file.name.clone()];

        if let Some(parent_id) = file.parent_id {
            Self::get_directory_path_recursive(parent_id, &store, &mut path);
        };

        let mut spaceless_path = path
            .iter()
            .map(|p| p.replace(" ", "%20"))
            .collect::<Vec<String>>();
        spaceless_path.reverse();
        spaceless_path.join("/")
    }

    pub fn save_candid(candid: String) {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        write(dir.join(format!("file_manager.did")), candid).expect("Write failed.");
    }

    pub fn build_asset_certificate_header(
        asset_hashes: &AssetHashes,
        path: String,
    ) -> Result<HeaderField, &'static str> {
        let certificate = data_certificate();

        match certificate {
            None => Err("No certificate found."),
            Some(certificate) => {
                Self::build_asset_certificate_header_impl(&certificate, asset_hashes, &path)
            }
        }
    }

    fn build_asset_certificate_header_impl(
        certificate: &Vec<u8>,
        asset_hashes: &AssetHashes,
        path: &String,
    ) -> Result<HeaderField, &'static str> {
        let witness = asset_hashes.tree.witness(path.as_bytes());
        let tree = labeled(b"http_assets", witness);

        let mut serializer = Serializer::new(vec![]);
        serializer.self_describe().unwrap();
        let result = tree.serialize(&mut serializer);

        match result {
            Err(_err) => Err("Failed to serialize a hash tree."),
            Ok(_serialize) => Ok(HeaderField(
                "IC-Certificate".to_string(),
                format!(
                    "certificate=:{}:, tree=:{}:",
                    base64::encode(&certificate),
                    base64::encode(&serializer.into_inner())
                ),
            )),
        }
    }

    pub fn update_certified_data(asset_hashes: &AssetHashes) {
        let prefixed_root_hash = &labeled_hash(b"http_assets", &asset_hashes.tree.root_hash());
        set_certified_data(&prefixed_root_hash[..]);
    }
}
