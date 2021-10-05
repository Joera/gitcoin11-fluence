/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

 use crate::curl_request;
 use crate::eth_utils::{check_response_string, get_nonce, BLOCK_NUMBER_TAGS};
 use crate::fce_results::JsonRpcResult;
 use crate::jsonrpc_helpers::{batch, Request};
 use chrono::Utc;
 use marine_rs_sdk::marine;
 use serde::{Deserialize, Deserializer, Serialize};
 use serde_json;
 use serde_json::Value;
 use std::sync::atomic::{AtomicUsize, Ordering};
 use bytes::Bytes;
 use hex_literal::hex;
 use ethers_core::types::{ Address, Signature, H256, U256, U64, TransactionRequest, PrivateKey, Bytes as EthBytes }; // Bytes
 use ethers_core::utils::keccak256;
 use ethers_core::utils::serialize;
 use ethers_core::secp256k1::Message;
 use serde_json::json;
 use std::{thread, time};
 

 use hex as hexx;
 
 use ethabi::{Token, encode };
 
 use ethabi_contract::use_contract;
 use ethabi_derive;
 use_contract!(ens_resolver, "resolver.json");
 
 const sender: &str = "0xF816Bf1d588100b6cea06B12CCe53fA81E81246A";
 const resolver: &str = "0xf6305c19e814d2a75429Fd637d01F7ee0E77d615";
 const provider: &str = "https://eth-goerli.alchemyapi.io/v2/Btuj6CK5zkBBtMDVJXM7zTa4RrC6hvCP";

 // iedere 15 min een nieuwe filter_id 
 // dan idere minuut? 


 #[marine] 
 pub fn check_filter(filter_id: &String) -> String {

  use ens_resolver::functions;
  
  let method = String::from("eth_getFilterChanges");
  let (from, to, chain_id, nonce) = init_stuff();

  let curl_args: Vec<String> = Request::new(method, vec!(), nonce).for_check(
    &String::from(filter_id),
    &provider.to_string()
  );

 // 
  let response = curl_request(curl_args);
  let response = String::from_utf8(response.stdout).unwrap();
  let res: serde_json::Value = serde_json::from_str(&response).unwrap();
  println!("{:?}", res);
  let mut id: i64; 
  let mut n: i64 = 0;

  if !res["result"].as_array().is_none() {

    for r in res["result"].as_array().unwrap() {

      let data = r["data"].as_str().unwrap().to_string();
      let mut i = 0;
      let mut s = String::from("");
      for c in data.chars() {
        if i > 65 { break; }
        if i > 63 {
            s.push(c);
        }
        i += 1;
      }
      n = hex_to_string(s);
    }
  } 
  n.to_string()
 }

 #[marine] 
 pub fn create_filter() -> String {

  let method = String::from("eth_newFilter");
  let (from, to, chain_id, nonce) = init_stuff();

  let from_block = String::from("0x1");
  let contract = String::from("0xf05f5bAB0984c22E0fc0c8875c51176Ec98D624e");
    // got this from etherscan 
  let topic = String::from("0x9a2e42fd6722813d69113e7d0079d3d940171428df7373df9c7f7617cfda2892");

  let curl_args: Vec<String> = Request::new(method, vec!(), nonce).as_clean(
    &from_block, 
    &contract, 
    &topic, 
    &provider.to_string()
  );

  println!("create: {:?}", curl_args);
  let response = curl_request(curl_args);
  let response = String::from_utf8(response.stdout).unwrap();
  let response: serde_json::Value = serde_json::from_str(&response).unwrap();
  let result = response["result"].as_str().unwrap();
  
  result.to_string()

 }


 fn init_stuff() -> (Address, Address, Option<u64>, u64) {

  let from = remove_zero_x(sender.to_string()).parse::<Address>().unwrap(); 
  let to = remove_zero_x(resolver.to_string()).parse::<Address>().unwrap();
  let chain_id = Some(4u64);
  let nonce = get_nonce();

  (from, to, chain_id, nonce)
 }
 

fn remove_zero_x(s: String) -> String {
 
  let mut i = 0;
  let mut numbah: String = "".to_string();
  for n in s.chars() {
      if i > 1 { numbah.push(n); }
      i = i + 1;
  }
  numbah
}


fn hex_to_int(hd: String) -> i64 {
  i64::from_str_radix(remove_zero_x(hd).as_str(), 16).unwrap()
}

fn hex_to_string(hd: String) -> i64 {
  i64::from_str_radix(hd.as_str(),16).unwrap()
}
