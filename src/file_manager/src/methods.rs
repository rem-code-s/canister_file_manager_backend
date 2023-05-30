use candid::{candid_method, Principal};
use ic_cdk::{caller, post_upgrade, pre_upgrade, query, storage, update};

use crate::{
    models::asset_models::{Asset, Id, NestedAssets},
    models::{
        asset_models::{AssetWithId, Permission},
        directory_models::DirectoryEntity,
        file_models::FileResponse,
        http_models::{
            AssetEncoding, HttpRequest, HttpResponse, StreamingCallbackHttpResponse,
            StreamingCallbackToken,
        },
        misc_models::{AssetHashes, Metadata},
    },
    store::{Store, ASSET_HASHES, STORE},
};

#[pre_upgrade]
fn pre_upgrade() {
    storage::stable_save((STORE.with(|s| s.clone()),)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (old_store,): (Store,) = storage::stable_restore().unwrap();
    let asset_hashes = AssetHashes::from(&old_store);
    ASSET_HASHES.with(|assets| *assets.borrow_mut() = asset_hashes.clone());
    STORE.with(|s| *s.borrow_mut() = old_store);

    Store::update_certified_data(&asset_hashes);
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
fn get_assets_tree(parent_id: Option<u64>, by_owner: bool) -> Vec<Asset> {
    Store::get_assets_tree(parent_id, if by_owner { Some(caller()) } else { None })
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
fn change_asset_name(name: String, asset: AssetWithId) -> Result<Asset, String> {
    Store::change_asset_name(name, asset)
}

#[update]
#[candid_method(update)]
fn change_asset_parent(parent_id: Option<Id>, asset: AssetWithId) -> Result<Asset, String> {
    Store::change_asset_parent(parent_id, asset)
}

#[update]
#[candid_method(update)]
fn change_asset_owner(owner: Principal, asset: AssetWithId) -> Result<Asset, String> {
    // Not implemented yet
    Err("Not implemented yet".to_string())
}

#[update]
#[candid_method(update)]
fn change_asset_permission(permission: Permission, asset: AssetWithId) -> Result<Asset, String> {
    Store::change_asset_permission(permission, asset)
}

#[update]
#[candid_method(update)]
fn delete_asset(asset: AssetWithId) -> Result<(), String> {
    Store::delete_asset(asset)
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
                    bytes_length: _file.size as u128,
                    hash: _file.hash.clone(),
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
