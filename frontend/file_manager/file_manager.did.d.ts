import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Asset = { 'File' : FileResponse } |
  { 'Directory' : DirectoryResponse };
export interface DirectoryEntity {
  'id' : bigint,
  'permission' : Permission,
  'updated_at' : bigint,
  'owner' : [] | [Principal],
  'name' : string,
  'created_at' : bigint,
  'parent_id' : [] | [bigint],
  'is_protected' : boolean,
}
export interface DirectoryResponse {
  'id' : bigint,
  'permission' : Permission,
  'updated_at' : bigint,
  'owner' : [] | [Principal],
  'name' : string,
  'created_at' : bigint,
  'children' : Vec,
  'parent_id' : [] | [bigint],
  'is_protected' : boolean,
}
export interface FileResponse {
  'id' : bigint,
  'permission' : Permission,
  'updated_at' : bigint,
  'owner' : [] | [Principal],
  'metadata' : [] | [string],
  'name' : string,
  'path' : string,
  'size' : bigint,
  'mime_type' : string,
  'created_at' : bigint,
  'parent_id' : [] | [bigint],
  'chunks' : BigUint64Array | bigint[],
  'is_protected' : boolean,
  'extension' : string,
}
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
}
export interface HttpResponse {
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
  'streaming_strategy' : [] | [StreamingStrategy],
  'status_code' : number,
}
export interface Metadata {
  'version' : string,
  'cycles' : bigint,
  'stable_memory' : bigint,
  'heap_memory' : bigint,
  'directory_count' : bigint,
  'file_count' : bigint,
  'files_combined_bytes' : bigint,
}
export interface NestedAssets { 'asset' : PostAsset, 'children' : Vec_1 }
export type Permission = { 'Private' : null } |
  { 'Origin' : Array<string> } |
  { 'Public' : null };
export type PostAsset = { 'File' : PostFile } |
  { 'None' : null } |
  { 'Directory' : PostDirectory };
export interface PostDirectory {
  'permission' : Permission,
  'name' : string,
  'children' : Array<Asset>,
  'parent_id' : [] | [bigint],
}
export interface PostFile {
  'permission' : Permission,
  'origin_path' : string,
  'metadata' : [] | [string],
  'name' : string,
  'size' : bigint,
  'mime_type' : string,
  'parent_id' : [] | [bigint],
  'chunk_count' : bigint,
  'extension' : string,
}
export type Result = { 'Ok' : Array<[FileResponse, string]> } |
  { 'Err' : [Array<Asset>, string] };
export type Result_1 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : DirectoryEntity } |
  { 'Err' : string };
export interface StreamingCallbackHttpResponse {
  'token' : [] | [StreamingCallbackToken],
  'body' : Uint8Array | number[],
}
export interface StreamingCallbackToken {
  'chunk_index' : bigint,
  'headers' : Array<[string, string]>,
  'file_id' : bigint,
}
export type StreamingStrategy = {
    'Callback' : {
      'token' : StreamingCallbackToken,
      'callback' : [Principal, string],
    }
  };
export type Vec = Array<
  {
      'File' : {
        'id' : bigint,
        'permission' : { 'Private' : null } |
          { 'Origin' : Array<string> } |
          { 'Public' : null },
        'updated_at' : bigint,
        'owner' : [] | [Principal],
        'metadata' : [] | [string],
        'name' : string,
        'path' : string,
        'size' : bigint,
        'mime_type' : string,
        'created_at' : bigint,
        'parent_id' : [] | [bigint],
        'chunks' : BigUint64Array | bigint[],
        'is_protected' : boolean,
        'extension' : string,
      }
    } |
    {
      'Directory' : {
        'id' : bigint,
        'permission' : { 'Private' : null } |
          { 'Origin' : Array<string> } |
          { 'Public' : null },
        'updated_at' : bigint,
        'owner' : [] | [Principal],
        'name' : string,
        'created_at' : bigint,
        'children' : Vec,
        'parent_id' : [] | [bigint],
        'is_protected' : boolean,
      }
    }
>;
export type Vec_1 = Array<
  {
    'asset' : {
        'File' : {
          'permission' : { 'Private' : null } |
            { 'Origin' : Array<string> } |
            { 'Public' : null },
          'origin_path' : string,
          'metadata' : [] | [string],
          'name' : string,
          'size' : bigint,
          'mime_type' : string,
          'parent_id' : [] | [bigint],
          'chunk_count' : bigint,
          'extension' : string,
        }
      } |
      { 'None' : null } |
      {
        'Directory' : {
          'permission' : { 'Private' : null } |
            { 'Origin' : Array<string> } |
            { 'Public' : null },
          'name' : string,
          'children' : Array<
            {
                'File' : {
                  'id' : bigint,
                  'permission' : { 'Private' : null } |
                    { 'Origin' : Array<string> } |
                    { 'Public' : null },
                  'updated_at' : bigint,
                  'owner' : [] | [Principal],
                  'metadata' : [] | [string],
                  'name' : string,
                  'path' : string,
                  'size' : bigint,
                  'mime_type' : string,
                  'created_at' : bigint,
                  'parent_id' : [] | [bigint],
                  'chunks' : BigUint64Array | bigint[],
                  'is_protected' : boolean,
                  'extension' : string,
                }
              } |
              {
                'Directory' : {
                  'id' : bigint,
                  'permission' : { 'Private' : null } |
                    { 'Origin' : Array<string> } |
                    { 'Public' : null },
                  'updated_at' : bigint,
                  'owner' : [] | [Principal],
                  'name' : string,
                  'created_at' : bigint,
                  'children' : Vec,
                  'parent_id' : [] | [bigint],
                  'is_protected' : boolean,
                }
              }
          >,
          'parent_id' : [] | [bigint],
        }
      },
    'children' : Vec_1,
  }
>;
export interface _SERVICE {
  'add_assets' : ActorMethod<[[] | [bigint], Array<NestedAssets>], Result>,
  'add_chunks' : ActorMethod<
    [Array<[bigint, Uint8Array | number[]]>],
    undefined
  >,
  'change_directory_permission' : ActorMethod<[bigint, Permission], Result_1>,
  'change_file_permission' : ActorMethod<[bigint, Permission], Result_1>,
  'create_directory' : ActorMethod<
    [string, Permission, [] | [bigint]],
    Result_2
  >,
  'delete_directory' : ActorMethod<[bigint], Result_1>,
  'delete_file' : ActorMethod<[bigint], Result_1>,
  'get_assets_tree' : ActorMethod<[[] | [bigint]], Array<Asset>>,
  'get_metadata' : ActorMethod<[], Metadata>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'http_request_streaming_callback' : ActorMethod<
    [StreamingCallbackToken],
    StreamingCallbackHttpResponse
  >,
}
