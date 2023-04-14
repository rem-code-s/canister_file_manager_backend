use candid::candid_method;
use ic_cdk::{post_upgrade, pre_upgrade, query, storage, update};

use crate::{
    models::asset_models::{Asset, Id, NestedAssets},
    models::{
        asset_models::{DirectoryEntity, FileResponse, Permission},
        http_models::{
            AssetEncoding, HttpRequest, HttpResponse, StreamingCallbackHttpResponse,
            StreamingCallbackToken,
        },
        misc_models::Metadata,
    },
    store::{Store, STORE},
};

#[pre_upgrade]
fn pre_upgrade() {
    storage::stable_save((STORE.with(|s| s.clone()),)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (old_store,): (Store,) = storage::stable_restore().unwrap();
    STORE.with(|s| *s.borrow_mut() = old_store);
}

#[test]
pub fn candid() {
    use candid::export_service;
    export_service!();
    use crate::models::misc_models::Metadata;
    Store::save_candid(__export_service());
}

#[query]
#[candid_method(query)]
fn get_assets_tree(parent_id: Option<u64>) -> Vec<Asset> {
    Store::get_assets_tree(parent_id)
}

#[update]
#[candid_method(update)]
fn delete_file(file_id: Id) -> Result<(), String> {
    Store::delete_file(file_id)
}

#[update]
#[candid_method(update)]
fn delete_directory(directory_id: u64) -> Result<(), String> {
    Store::delete_directory(directory_id)
}

#[update]
#[candid_method(update)]
fn add_assets(
    parent_id: Option<Id>,
    assets: Vec<NestedAssets>,
) -> Result<Vec<(FileResponse, String)>, (Vec<Asset>, String)> {
    Store::add_assets(parent_id, assets)
}

#[update]
#[candid_method(update)]
fn add_chunks(data: Vec<(Id, Vec<u8>)>) {
    Store::add_chunks(data)
}

#[update]
#[candid_method(update)]
fn create_directory(
    name: String,
    permission: Permission,
    parent_id: Option<Id>,
) -> Result<DirectoryEntity, String> {
    Store::create_directory(name, permission, parent_id)
}

#[update]
#[candid_method(update)]
fn change_file_permission(file_id: Id, permission: Permission) -> Result<(), String> {
    Store::change_file_permission(file_id, permission)
}

#[update]
#[candid_method(update)]
fn change_directory_permission(directory_id: Id, permission: Permission) -> Result<(), String> {
    Store::change_directory_permission(directory_id, permission)
}

#[query]
#[candid_method(query)]
fn get_metadata() -> Metadata {
    Store::get_metadata()
}

#[query]
#[candid_method(query)]
fn http_request(req: HttpRequest) -> HttpResponse {
    Store::http_request(req)
}

#[query]
#[candid_method(query)]
fn http_request_streaming_callback(data: StreamingCallbackToken) -> StreamingCallbackHttpResponse {
    STORE.with(|store| -> StreamingCallbackHttpResponse {
        let store = store.borrow();
        let file = store.files.get(&data.file_id);
        match file {
            Some(_file) => {
                let encoding = AssetEncoding {
                    content_chunks: _file.chunks.clone(),
                    total_length: _file.size as u128,
                };

                let body = store
                    .chunks
                    .get(&encoding.content_chunks[data.chunk_index])
                    .unwrap()
                    .clone();

                StreamingCallbackHttpResponse {
                    token: Store::create_token(
                        &data.file_id,
                        data.chunk_index,
                        &encoding,
                        &data.headers,
                    ),
                    body,
                }
            }
            None => StreamingCallbackHttpResponse {
                token: None,
                body: vec![],
            },
        }
    })
}
