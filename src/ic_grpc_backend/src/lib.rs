mod types;

use serde_json::*;

use std::borrow::Cow;

use types::*;

#[ic_cdk::query]
pub fn http_request(req: HttpRequest) -> HttpResponse {
    let bytes = req.body.into_vec();
    let rpc_msg = std::str::from_utf8(&bytes).unwrap();

    let req: RpcRequest = serde_json::from_str(rpc_msg).unwrap();

    ic_cdk::print(format!("{:#?}", req));

    let (result, upgrade) = match req.method.as_str() {
        "hello" => (Value::from(String::from("world")), false),
        _ => (Value::from(String::from("")), false),
    };

    let resp = serde_json::to_string(&RpcResponse {
        jsonrpc: String::from("2.0"),
        result,
        id: req.id,
    })
    .unwrap();

    if !upgrade {
        ic_cdk::print(format!("{:#?}", resp));
    }

    HttpResponse {
        status_code: 200,
        upgrade: Some(upgrade),
        headers: [(
            String::from("content-type"),
            String::from("application/json"),
        )]
        .to_vec(),
        body: Cow::Owned(serde_bytes::ByteBuf::from(resp.as_bytes())),
        streaming_strategy: None,
    }
}