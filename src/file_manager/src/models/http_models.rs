use candid::{CandidType, Deserialize, Func};

#[derive(CandidType, Deserialize, Clone)]
pub struct HeaderField(pub String, pub String);

#[derive(CandidType, Deserialize, Clone)]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub headers: Vec<HeaderField>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct HttpResponse {
    pub body: Vec<u8>,
    pub headers: Vec<HeaderField>,
    pub status_code: u16,
    pub streaming_strategy: Option<StreamingStrategy>,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum StreamingStrategy {
    Callback {
        token: StreamingCallbackToken,
        callback: Func,
    },
}

#[derive(CandidType, Deserialize, Clone)]
pub struct StreamingCallbackToken {
    pub file_id: u64,
    pub headers: Vec<HeaderField>,
    pub chunk_index: usize,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct StreamingCallbackHttpResponse {
    pub body: Vec<u8>,
    pub token: Option<StreamingCallbackToken>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct AssetEncoding {
    pub content_chunks: Vec<u64>,
    pub total_length: u128,
}

#[derive(Clone, CandidType, Deserialize)]
pub struct PathEntry {
    pub match_path: Vec<String>,
    pub response: HttpResponse,
}
