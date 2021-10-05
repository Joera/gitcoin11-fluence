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

 use marine_rs_sdk::marine;

 use hex_literal::hex;
 use ethers_core::types::{ Address, Signature, H256, U256, U64, TransactionRequest, PrivateKey, Bytes as EthBytes }; // Bytes
 use ethers_core::utils::serialize;
 use ethers_core::secp256k1::Message;

 use serde_json::json;
 use serde_json::Value;
 
 use hex as hexx;
 
 use ethabi::{Token, encode };
 use ethabi_contract::use_contract;
 use ethabi_derive;
 use_contract!(ens_resolver, "resolver.json");
 

 const sender: &str = "0xF816Bf1d588100b6cea06B12CCe53fA81E81246A";
 const pkey: &str = "ee6ae6900f8a762f12d34fedfdbeab47f27e247ca2fd80160d72700f04ab88e5";
 const resolver: &str = "0xf6305c19e814d2a75429Fd637d01F7ee0E77d615";
 //const url: &str = "https://eth-rinkeby.alchemyapi.io/v2/oDMoyeai5fTVQxfpjKQVJM3ltl1ap9z7";

 #[marine]
 pub fn get_record(key: String, eth_provider_url: String) -> String {

    use ens_resolver::functions;
    let method = String::from("eth_call");
    let (private_key, from, to, chain_id, nonce) = init_stuff();
    let ens_node =  hex!("ba74231ea40fda8e9d6c31f3c5c9941d4ca54cde0f5829414e8232cca7e919a1");

    let input_data = functions::text::encode_input(ens_node, key);
    let data = format!("0x{}", hexx::encode(input_data)); 

    let curl_args: Vec<String> = Request::new(method, vec!(), nonce).as_json(
      &sender.to_string(), 
      &resolver.to_string(), 
      &data, 
      &eth_provider_url
    );

    let response = curl_request(curl_args);
    let response = String::from_utf8(response.stdout).unwrap();
    let response: serde_json::Value = serde_json::from_str(&response).unwrap();
    let result = &response["result"].as_str().unwrap();
    let result = hexx::decode(remove_zero_x(result.to_string())).unwrap();
    let result = functions::text::decode_output(&result).unwrap();
  
    result
 }


 #[marine]
 pub fn update_record(key: String, value: String, eth_provider_url: String) -> String {

    use ens_resolver::functions;
    let method = String::from("eth_sendRawTransaction");
    let (private_key,from,to,chain_id,nonce) = init_stuff();
    let ens_node =  hex!("ba74231ea40fda8e9d6c31f3c5c9941d4ca54cde0f5829414e8232cca7e919a1");
    
    let mut tx_count = remove_zero_x(eth_get_transaction_count(&sender.to_string(), &eth_provider_url).result);
    let tx_count = tx_count.parse::<U256>().unwrap();
     
    let input_data = functions::set_text::encode_input(ens_node, key, value);
    let input_data_bytes: EthBytes = input_data.into();
   
     let mut tx_request = TransactionRequest {
         from: Some(from.into()),
         to: Some(to.into()),
         gas: Some(80_000.into()),
         gas_price: Some(hex_to_int(eth_gas_price(&eth_provider_url).result).into()),
         value: None,
         data : Some(input_data_bytes), 
         nonce: Some(tx_count)
     };
  
     let sighash = tx_request.sighash(chain_id);
     let message = Message::parse_slice(sighash.as_bytes()).expect("hash is non-zero 32-bytes; qed");
     let signature = private_key.sign_with_eip155(&message, chain_id);
     let tx = tx_request.rlp_signed(&signature);
     // let tx = private_key.sign_transaction(tx_request,chain_id).unwrap(); */
     // solution was to have rpl in bytes and use ethers-core::utils::serialize to create (long) hex
     let tx_string = String::from(serialize(&tx).as_str().unwrap());
     let params: Vec<String> = vec![tx_string];
 
     let curl_args: Vec<String> = Request::new(method, params, nonce).as_vec(&eth_provider_url);
     let response = curl_request(curl_args);
     
     String::from_utf8(response.stdout).unwrap()
 }

 fn init_stuff() -> (PrivateKey, Address, Address, Option<u64>, u64) {


  let private_key: PrivateKey = pkey.parse().unwrap();
  let from = remove_zero_x(sender.to_string()).parse::<Address>().unwrap(); 
  let to = remove_zero_x(resolver.to_string()).parse::<Address>().unwrap();
  let chain_id = Some(4u64);
  let nonce = get_nonce();

  (private_key, from, to, chain_id, nonce)
 }
 

fn eth_gas_price(eth_provider_url: &String) -> JsonRpcResult {
     let method = String::from("eth_gasPrice");
     let id = get_nonce();
 
     let params: Vec<String> = vec![];
     let curl_args: Vec<String> = Request::new(method, params, id).as_vec(eth_provider_url);
     let response = curl_request(curl_args);
     let response = String::from_utf8(response.stdout).unwrap();
     check_response_string(response, &id)
 }
 

fn eth_get_transaction_count(account: &String, eth_provider_url: &String) -> JsonRpcResult {
     let method = String::from("eth_getTransactionCount");
     let block_identifier = String::from("latest");
     let id = get_nonce();
 
     let params: Vec<String> = vec![account.to_string(), block_identifier];
     let curl_args: Vec<String> = Request::new(method, params, id).as_vec(eth_provider_url);
     let response = curl_request(curl_args);
     let response = String::from_utf8(response.stdout).unwrap();
     check_response_string(response, &id)
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
