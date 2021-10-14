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
use crate::{file_put, file_get};
use crate::auth::{is_owner};

use marine_rs_sdk::marine;

use std::str;
use libsecp256k1::{SecretKey, PublicKey};
use rand::rngs::OsRng;
use tiny_keccak::{Keccak, Hasher};
use hex;

use serde_json::json;
use serde_json::Value;

use std::cell::RefCell;
use ethers_core::types::{ Address, Signature, U256, U64, TransactionRequest, PrivateKey, Bytes as EthBytes }; // Bytes
use ethers_core::utils::serialize;
use ethers_core::secp256k1::Message;

thread_local!(
    static ADDRESS: RefCell<String> = RefCell::new(String::from(""));
    static KEY: RefCell<String> = RefCell::new(String::from(""));
);

#[marine]
#[derive(Debug)]
pub struct Gresult {
    pub envs: Vec<String>,
    pub err_str: String,
}

 
 #[marine]
 pub fn initiate() -> Vec<String> {
  
  let skey = SecretKey::random(&mut OsRng);
  let pkey = PublicKey::from_secret_key(&skey);

  // find way to store skey 
  let secret_string = hex::encode(skey.serialize());
  
  // create eth address from public key 
  let mut keccak = Keccak::v256();
  let mut output = [0; 64];
  keccak.update(&pkey.serialize());
  keccak.finalize(&mut output);
  let hexed = hex::encode(output);
  let v: Vec<char> = hexed.chars().rev().take(40).collect();
  let s: String = v.iter().rev().collect();
  let address = format!("0x{}", s); 

  ADDRESS.with(|address_cell| {
    address_cell.replace(address.clone());
  });

  KEY.with(|key_cell| {
        key_cell.replace(secret_string.clone());
  });
  

  // write_to_file(address.clone(), secret_string);

  vec!(address, secret_string)

 } 

 #[marine]
 pub fn test() -> String {

    let mut address: String = String::from("");

        ADDRESS.with(|address_cell| {
            address = address_cell.borrow_mut().to_string();
        });


    address

}

 #[marine]
 pub fn sign_transaction_request(tx_request_string: &str) -> String { // tx_request: TransactionRequest
    
    let chain_id = Some(4u64);
    let mut tx_string = String::from("");

    let tx_request: TransactionRequest = serde_json::from_str(tx_request_string).unwrap();

    KEY.with(|key_cell| {
        
        let key = key_cell.borrow_mut();
        let private_key: PrivateKey = key.parse().unwrap();

        let sighash = tx_request.sighash(chain_id);
        let message = Message::parse_slice(sighash.as_bytes()).expect("hash is non-zero 32-bytes; qed");
        let signature = private_key.sign_with_eip155(&message, chain_id);
        let tx = tx_request.rlp_signed(&signature);
    
        // solution was to have rpl in bytes and use ethers-core::utils::serialize to create (long) hex
        tx_string = String::from(serialize(&tx).as_str().unwrap())
    });

    tx_string

  }

/*

 fn write_to_file(address: String, secret_string: String) {

    let file_name = String::from("env.json");

    let mut map = serde_json::Map::new();
    map.insert(String::from("a"), serde_json::Value::String(address));
    map.insert(String::from("k"), serde_json::Value::String(secret_string)); 
    let obj = serde_json::Value::Object(map);

    file_put(file_name, obj.to_string().into_bytes());
 }

 #[marine]
 pub fn env() -> Gresult {

   let file_name = String::from("env.json");
   let obj = file_get(file_name);
   let json: serde_json::Value = serde_json::from_slice(&obj).unwrap();

   let address = json["a"].as_str().unwrap();
   let key = json["k"].as_str().unwrap();

   let envs: Vec<String> = vec!(String::from(address), String::from(key));

   match is_owner() {
      true => Gresult {
          envs: envs,
          err_str: "".into(),
      },
      false => Gresult {
          envs: vec!(String::from("")),
          err_str: "You are not the owner".into(),
      },
  }
 }

*/

 #[marine]
 pub fn make_call(key: String, eth_provider_url: String) -> String {

  

    let curl_args = vec!();
    
    //Request::new(method, vec!(), nonce).as_json( );

    let response = curl_request(curl_args);
    let response = String::from_utf8(response.stdout).unwrap();
    let response: serde_json::Value = serde_json::from_str(&response).unwrap();
    let result = &response["result"].as_str().unwrap();
  
    String::from("hi")
 }

 // 