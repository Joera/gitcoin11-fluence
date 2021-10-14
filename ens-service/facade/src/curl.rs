use crate::curl_request;
use crate::eth_utils::{check_response_string };
use crate::fce_results::JsonRpcResult;
// use crate::jsonrpc_helpers::{ Request};

pub fn request(args: Vec<String>, id: u64) -> JsonRpcResult {

    let response = curl_request(args);
    let response = String::from_utf8(response.stdout).unwrap();
    check_response_string(response, &id)

} 