use candid::{CandidType, Deserialize, Func};
use std::borrow::Cow;
use serde::Serialize;
use serde_bytes::{ByteBuf, Bytes};
use serde_json::*;

#[derive(Clone, Debug, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<serde_json::Value>,
    pub id: serde_json::Value,
}

#[derive(Clone, Debug, Serialize)]
pub struct RpcResponse {
    pub jsonrpc: String,
    pub result: Value,
    pub id: serde_json::Value,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Token {}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum StreamingStrategy {
    Callback { callback: Func, token: Token },
}

pub type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: ByteBuf,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub upgrade: Option<bool>,
    pub headers: Vec<HeaderField>,
    pub body: Cow<'static, Bytes>,
    pub streaming_strategy: Option<StreamingStrategy>,
}