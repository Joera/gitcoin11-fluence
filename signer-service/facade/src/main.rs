#![allow(
    non_snake_case,
    unused_variables,
    unused_imports,
    unused_parens,
    unused_mut
)]

mod keccak;
mod auth;
mod keys;
mod memory;
mod transactions;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;

use marine_rs_sdk::CallParameters;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().ok();
}


#[marine]
pub fn initiate() -> Vec<String> {
    
    if memory::unitialized() {
        // store particle id or reference to recurring aqua script 
        keys::new()
    } else {
        vec!(String::from("0x"))
    }
}

#[marine]
pub fn sign_tx(tx_request_string: &str, chain_id: u64) -> String {
    // only the recurring aqua script ... how to check? particle? 
    transactions::sign(tx_request_string, chain_id)
}

#[marine] 
pub fn address() -> String {

    memory::address()
}

