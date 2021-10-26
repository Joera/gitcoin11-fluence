use crate::memory;

use serde_json::json;
use ethers_core::types::{ TransactionRequest, PrivateKey }; // Bytes
use ethers_core::utils::serialize;
use ethers_core::secp256k1::Message;

pub fn sign(tx_request_string: &str, chain_id: u64) -> String { 

    let mut tx_string = String::from("");
    let tx_request: TransactionRequest = serde_json::from_str(tx_request_string).unwrap();
    let private_key: PrivateKey = memory::key().parse().unwrap();

    let sighash = tx_request.sighash(Some(chain_id));
    let message = Message::parse_slice(sighash.as_bytes()).expect("hash is non-zero 32-bytes; qed");
    let signature = private_key.sign_with_eip155(&message, Some(chain_id));
    let tx = tx_request.rlp_signed(&signature);

    // solution was to have rpl in bytes and use ethers-core::utils::serialize to create (long) hex
    String::from(serialize(&tx).as_str().unwrap())
}