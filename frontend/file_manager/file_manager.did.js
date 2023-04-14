export const idlFactory = ({ IDL }) => {
  const Vec = IDL.Rec();
  const Vec_1 = IDL.Rec();
  const Permission = IDL.Variant({
    'Private' : IDL.Null,
    'Origin' : IDL.Vec(IDL.Text),
    'Public' : IDL.Null,
  });
  const PostFile = IDL.Record({
    'permission' : Permission,
    'origin_path' : IDL.Text,
    'metadata' : IDL.Opt(IDL.Text),
    'name' : IDL.Text,
    'size' : IDL.Nat64,
    'mime_type' : IDL.Text,
    'parent_id' : IDL.Opt(IDL.Nat64),
    'chunk_count' : IDL.Nat64,
    'extension' : IDL.Text,
  });
  const FileResponse = IDL.Record({
    'id' : IDL.Nat64,
    'permission' : Permission,
    'updated_at' : IDL.Nat64,
    'owner' : IDL.Opt(IDL.Principal),
    'metadata' : IDL.Opt(IDL.Text),
    'name' : IDL.Text,
    'path' : IDL.Text,
    'size' : IDL.Nat64,
    'mime_type' : IDL.Text,
    'created_at' : IDL.Nat64,
    'parent_id' : IDL.Opt(IDL.Nat64),
    'chunks' : IDL.Vec(IDL.Nat64),
    'is_protected' : IDL.Bool,
    'extension' : IDL.Text,
  });
  Vec.fill(
    IDL.Vec(
      IDL.Variant({
        'File' : IDL.Record({
          'id' : IDL.Nat64,
          'permission' : IDL.Variant({
            'Private' : IDL.Null,
            'Origin' : IDL.Vec(IDL.Text),
            'Public' : IDL.Null,
          }),
          'updated_at' : IDL.Nat64,
          'owner' : IDL.Opt(IDL.Principal),
          'metadata' : IDL.Opt(IDL.Text),
          'name' : IDL.Text,
          'path' : IDL.Text,
          'size' : IDL.Nat64,
          'mime_type' : IDL.Text,
          'created_at' : IDL.Nat64,
          'parent_id' : IDL.Opt(IDL.Nat64),
          'chunks' : IDL.Vec(IDL.Nat64),
          'is_protected' : IDL.Bool,
          'extension' : IDL.Text,
        }),
        'Directory' : IDL.Record({
          'id' : IDL.Nat64,
          'permission' : IDL.Variant({
            'Private' : IDL.Null,
            'Origin' : IDL.Vec(IDL.Text),
            'Public' : IDL.Null,
          }),
          'updated_at' : IDL.Nat64,
          'owner' : IDL.Opt(IDL.Principal),
          'name' : IDL.Text,
          'created_at' : IDL.Nat64,
          'children' : Vec,
          'parent_id' : IDL.Opt(IDL.Nat64),
          'is_protected' : IDL.Bool,
        }),
      })
    )
  );
  const DirectoryResponse = IDL.Record({
    'id' : IDL.Nat64,
    'permission' : Permission,
    'updated_at' : IDL.Nat64,
    'owner' : IDL.Opt(IDL.Principal),
    'name' : IDL.Text,
    'created_at' : IDL.Nat64,
    'children' : Vec,
    'parent_id' : IDL.Opt(IDL.Nat64),
    'is_protected' : IDL.Bool,
  });
  const Asset = IDL.Variant({
    'File' : FileResponse,
    'Directory' : DirectoryResponse,
  });
  const PostDirectory = IDL.Record({
    'permission' : Permission,
    'name' : IDL.Text,
    'children' : IDL.Vec(Asset),
    'parent_id' : IDL.Opt(IDL.Nat64),
  });
  const PostAsset = IDL.Variant({
    'File' : PostFile,
    'None' : IDL.Null,
    'Directory' : PostDirectory,
  });
  Vec_1.fill(
    IDL.Vec(
      IDL.Record({
        'asset' : IDL.Variant({
          'File' : IDL.Record({
            'permission' : IDL.Variant({
              'Private' : IDL.Null,
              'Origin' : IDL.Vec(IDL.Text),
              'Public' : IDL.Null,
            }),
            'origin_path' : IDL.Text,
            'metadata' : IDL.Opt(IDL.Text),
            'name' : IDL.Text,
            'size' : IDL.Nat64,
            'mime_type' : IDL.Text,
            'parent_id' : IDL.Opt(IDL.Nat64),
            'chunk_count' : IDL.Nat64,
            'extension' : IDL.Text,
          }),
          'None' : IDL.Null,
          'Directory' : IDL.Record({
            'permission' : IDL.Variant({
              'Private' : IDL.Null,
              'Origin' : IDL.Vec(IDL.Text),
              'Public' : IDL.Null,
            }),
            'name' : IDL.Text,
            'children' : IDL.Vec(
              IDL.Variant({
                'File' : IDL.Record({
                  'id' : IDL.Nat64,
                  'permission' : IDL.Variant({
                    'Private' : IDL.Null,
                    'Origin' : IDL.Vec(IDL.Text),
                    'Public' : IDL.Null,
                  }),
                  'updated_at' : IDL.Nat64,
                  'owner' : IDL.Opt(IDL.Principal),
                  'metadata' : IDL.Opt(IDL.Text),
                  'name' : IDL.Text,
                  'path' : IDL.Text,
                  'size' : IDL.Nat64,
                  'mime_type' : IDL.Text,
                  'created_at' : IDL.Nat64,
                  'parent_id' : IDL.Opt(IDL.Nat64),
                  'chunks' : IDL.Vec(IDL.Nat64),
                  'is_protected' : IDL.Bool,
                  'extension' : IDL.Text,
                }),
                'Directory' : IDL.Record({
                  'id' : IDL.Nat64,
                  'permission' : IDL.Variant({
                    'Private' : IDL.Null,
                    'Origin' : IDL.Vec(IDL.Text),
                    'Public' : IDL.Null,
                  }),
                  'updated_at' : IDL.Nat64,
                  'owner' : IDL.Opt(IDL.Principal),
                  'name' : IDL.Text,
                  'created_at' : IDL.Nat64,
                  'children' : Vec,
                  'parent_id' : IDL.Opt(IDL.Nat64),
                  'is_protected' : IDL.Bool,
                }),
              })
            ),
            'parent_id' : IDL.Opt(IDL.Nat64),
          }),
        }),
        'children' : Vec_1,
      })
    )
  );
  const NestedAssets = IDL.Record({ 'asset' : PostAsset, 'children' : Vec_1 });
  const Result = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(FileResponse, IDL.Text)),
    'Err' : IDL.Tuple(IDL.Vec(Asset), IDL.Text),
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const DirectoryEntity = IDL.Record({
    'id' : IDL.Nat64,
    'permission' : Permission,
    'updated_at' : IDL.Nat64,
    'owner' : IDL.Opt(IDL.Principal),
    'name' : IDL.Text,
    'created_at' : IDL.Nat64,
    'parent_id' : IDL.Opt(IDL.Nat64),
    'is_protected' : IDL.Bool,
  });
  const Result_2 = IDL.Variant({ 'Ok' : DirectoryEntity, 'Err' : IDL.Text });
  const Metadata = IDL.Record({
    'version' : IDL.Text,
    'cycles' : IDL.Nat64,
    'stable_memory' : IDL.Nat64,
    'heap_memory' : IDL.Nat64,
    'directory_count' : IDL.Nat64,
    'file_count' : IDL.Nat64,
    'files_combined_bytes' : IDL.Nat64,
  });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const StreamingCallbackToken = IDL.Record({
    'chunk_index' : IDL.Nat64,
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'file_id' : IDL.Nat64,
  });
  const StreamingStrategy = IDL.Variant({
    'Callback' : IDL.Record({
      'token' : StreamingCallbackToken,
      'callback' : IDL.Func([], [], []),
    }),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'streaming_strategy' : IDL.Opt(StreamingStrategy),
    'status_code' : IDL.Nat16,
  });
  const StreamingCallbackHttpResponse = IDL.Record({
    'token' : IDL.Opt(StreamingCallbackToken),
    'body' : IDL.Vec(IDL.Nat8),
  });
  return IDL.Service({
    'add_assets' : IDL.Func(
        [IDL.Opt(IDL.Nat64), IDL.Vec(NestedAssets)],
        [Result],
        [],
      ),
    'add_chunks' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Nat64, IDL.Vec(IDL.Nat8)))],
        [],
        [],
      ),
    'change_directory_permission' : IDL.Func(
        [IDL.Nat64, Permission],
        [Result_1],
        [],
      ),
    'change_file_permission' : IDL.Func(
        [IDL.Nat64, Permission],
        [Result_1],
        [],
      ),
    'create_directory' : IDL.Func(
        [IDL.Text, Permission, IDL.Opt(IDL.Nat64)],
        [Result_2],
        [],
      ),
    'delete_directory' : IDL.Func([IDL.Nat64], [Result_1], []),
    'delete_file' : IDL.Func([IDL.Nat64], [Result_1], []),
    'get_assets_tree' : IDL.Func(
        [IDL.Opt(IDL.Nat64)],
        [IDL.Vec(Asset)],
        ['query'],
      ),
    'get_metadata' : IDL.Func([], [Metadata], ['query']),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'http_request_streaming_callback' : IDL.Func(
        [StreamingCallbackToken],
        [StreamingCallbackHttpResponse],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
