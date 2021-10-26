use crate::curl_request;
use crate::jsonrpc_helpers::{ Request };
use crate::fce_results::JsonRpcResult;
use std::sync::atomic::{AtomicUsize, Ordering};
use ethers_core::types::{U256, Address, NameOrAddress, H160};


use ethabi_contract::use_contract;
use ethabi_derive;
use_contract!(ens_resolver, "resolver.json");

pub static NONCE_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub fn get_nonce() -> u64 {
    NONCE_COUNTER.fetch_add(1, Ordering::SeqCst) as u64
}

pub fn address_for_from(readable: &str) -> H160 {

    let address = remove_zero_x(readable.to_string()).parse::<Address>().unwrap();

    address.into() 
}


pub fn address_for_to(readable: &str) -> NameOrAddress {

    let address = remove_zero_x(readable.to_string()).parse::<Address>().unwrap();

    address.into() 
}
  

pub fn ethUnformattedDataToCid(response: JsonRpcResult) -> String {

    // from: 0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000003b62616679726569686e3637616c3374326b7977657961786d7a35677164726a73736a6a786a637a74776c376736696e7a78767977337274677776790000000000
    // to: bafyreihn67al3t2kyweyaxmz5gqdrjssjjxjcztwl7g6inzxvyw3rtgwvy
    let result = hex::decode(remove_zero_x(response.result.to_string())).unwrap();
    ens_resolver::functions::text::decode_output(&result).unwrap()

}

pub fn get_transaction_count(account: &String, eth_provider_url: &String) -> U256 {
     
    let nonce = get_nonce();
      
    let curl_args: Vec<String> = Request::new(

      eth_provider_url.to_string(), 
      String::from("eth_getTransactionCount"), 
      nonce

    ).format(vec![

      account.to_string(),
      String::from("latest")
    ]);
     
     let response = curl_request(curl_args);
     let response = String::from_utf8(response.stdout).unwrap();
     let response : JsonRpcResult = check_response_string(response, &nonce);
     println!("{:?}", response);

     let tx_count = remove_zero_x(response.result);

     tx_count.parse::<U256>().unwrap()
}

pub fn gas_price_for_raw_transaction(eth_provider_url: &String) -> U256 {

    let response: JsonRpcResult = gas_price(eth_provider_url);
    
    hex_to_int(response.result).into()
}

fn gas_price(eth_provider_url: &String) -> JsonRpcResult {

    let nonce = get_nonce();
     
    let curl_args: Vec<String> = Request::new(
      
      eth_provider_url.to_string(), 
      String::from("eth_gasPrice"),
      nonce
    ).format(vec![

    ]);

    let response = curl_request(curl_args);
    let response = String::from_utf8(response.stdout).unwrap();
    check_response_string(response, &nonce)
}
 

pub fn check_response_string(response: String, id: &u64) -> JsonRpcResult {
    if response.len() == 0 {
        let err_msg = "{\"jsonrpc\":\"$V\",\"id\":$ID,\"error\":{\"code\":-32700,\"message\":Curl connection failed}}";
        let err_msg = err_msg.replace("$ID", &id.to_string());
        return JsonRpcResult::from(Result::from(Err(err_msg)));
    }

    match response.contains("error") {
        true => JsonRpcResult::from(Result::from(Err(response))),
        false => JsonRpcResult::from(Result::from(Ok(response))),
    }
}

fn hex_to_int(hd: String) -> i64 {
    i64::from_str_radix(remove_zero_x(hd).as_str(), 16).unwrap()
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


