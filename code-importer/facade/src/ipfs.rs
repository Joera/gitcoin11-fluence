use crate::curl_request;
use chrono::Utc;
use marine_rs_sdk::marine;
use serde::{ Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;

use std::collections::BTreeMap;
use indexmap::IndexMap;


#[marine]
fn get_wasm_from_ipfs(cid: String, remote_ipfs: String) {

    let url = format!("http://{}:8080/api/v0/get/{}", remote_ipfs, cid);

    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("GET"),
        url
    ];

    let response = curl_request(curl_args);   
    String::from_utf8(response.stdout).unwrap();
}

