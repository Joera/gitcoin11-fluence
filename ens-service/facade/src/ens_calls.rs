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
use crate::ens_namehash::convert;
use crate::curl::request;
use crate::eth_utils::{
  check_response_string, 
  get_nonce, 
  ethUnformattedDataToCid, 
  gas_price_for_raw_transaction, 
  get_transaction_count, 
  address_for_from,
  address_for_to
};

use crate::fce_results::JsonRpcResult;
use crate::jsonrpc_helpers::{ Request };
//use crate::eip1559_types::{AccessList};

 use marine_rs_sdk::marine;

 use ethers_core::types::{ Address, Signature, U256, U64, TransactionRequest, PrivateKey, Bytes as EthBytes }; // Bytes
 use ethers_core::utils::serialize;
 use ethers_core::secp256k1::Message;

 use serde::Serialize as serde_serialize;
 use serde_json::json;
 use serde_json::Value;
 
 use hex::{encode as hex_encode};

 use ethabi::{Token, encode, ethereum_types::H256};
 use ethabi_contract::use_contract;
 use ethabi_derive;
 use_contract!(ens_resolver, "resolver.json");

 const RESOLVER: &str = "0xf6305c19e814d2a75429Fd637d01F7ee0E77d615";


 #[marine]
 pub fn get_record(key: String, eth_provider_url: String, ens_domain: String) -> String {

    let nonce = get_nonce();

    let curl_args: Vec<String> = Request::new(
      
      eth_provider_url, 
      String::from("eth_call"), 
      nonce
    
    ).format_call(

      &RESOLVER.to_string(), 
      &format!(
        "0x{}", 
        hex_encode(
          ens_resolver::functions::text::encode_input(
            H256::from_slice(&convert(&ens_domain.as_str())[..]), // ens_node
            key // property name
          )
        )
      )
    );

    let response: JsonRpcResult = request(curl_args, nonce);
    
    ethUnformattedDataToCid(response)

 }


 #[marine]
 pub fn prepare_update(eth_provider_url: &String, sender: String, ens_domain: String, key: String, value: String) -> String {

    // build contract specific part of the transaction 
    let input_data_bytes: EthBytes = ens_resolver::functions::set_text::encode_input(
      
      H256::from_slice(&convert(&ens_domain.as_str())[..]), // ens_node
      key,  // property name 
      value // property value
    
    ).into(); 
    
    // build the transaction 
    let mut tx_request = TransactionRequest {
         from: None,
         to: Some(address_for_to(&RESOLVER)),
         gas: Some(500_000.into()),
         gas_price: Some(gas_price_for_raw_transaction(&eth_provider_url)),
         value: None,
         data : Some(input_data_bytes), 
         nonce: Some(get_transaction_count(&sender.to_string(), &eth_provider_url) + 0)
    };

    /* waiting to re-use ethers types ... requires marine to work with serde 

    let mut eip1559_tx_request = Eip1559TransactionRequest {
      from: None,
      to:Some(address_for_to(&RESOLVER)),
      gas: None,
      value: None,
      data : Some(input_data_bytes), 
      nonce: Some(get_transaction_count(&sender.to_string(), &eth_provider_url) + 2),
      access_list: vec!(),
      max_priority_fee_per_gas: Some(10_000.into()),
      max_fee_per_gas: Some(250_000.into())
    }; */

    serde_json::to_string(&tx_request).unwrap()
  } 

  #[marine]
 pub fn prepare_update_with_u64(eth_provider_url: &String, sender: String, ens_domain: String, key: String, value: u64 ) -> String {

    // build contract specific part of the transaction 
    let input_data_bytes: EthBytes = ens_resolver::functions::set_text::encode_input(
      
      H256::from_slice(&convert(&ens_domain.as_str())[..]), // ens_node
      key,  // property name 
      value.to_string() // property value
    
    ).into(); 
    
    // build the transaction 
    let mut tx_request = TransactionRequest {
         from: None,
         to: Some(address_for_to(&RESOLVER)),
         gas: Some(200_000.into()),
         gas_price: Some(gas_price_for_raw_transaction(&eth_provider_url)),
         value: None,
         data : Some(input_data_bytes), 
         nonce: Some(get_transaction_count(&sender.to_string(), &eth_provider_url) + 0)
    };

    serde_json::to_string(&tx_request).unwrap()
  } 


  #[marine]
  pub fn make_update_request(eth_provider_url: String, tx_string: String) -> JsonRpcResult {
 
    let nonce = get_nonce();

    let curl_args: Vec<String> = Request::new(
      
      eth_provider_url, 
      String::from("eth_sendRawTransaction"), 
      nonce
    
    ).format(
      vec![tx_string]
    );

    request(curl_args, nonce)
      
 }