use crate::keccak::Keccak256;
use crate::memory::{store};

use std::str;
use std::convert::TryInto;
use libsecp256k1::{SecretKey, PublicKey};
use rand::rngs::OsRng;
use hex as hex;

pub fn new() -> Vec<String> {

  let secret_key = SecretKey::random(&mut OsRng);
  let public_key = PublicKey::from_secret_key(&secret_key);
  let secret_string = format_secret_key(secret_key);
  let address = format_address(public_key);
    
  store(&address, &secret_string);
  vec!(address, secret_string)
}

fn format_address(public_key: PublicKey) -> String {

  let hash = public_key.serialize()[1..65].keccak256();
  let trimmed: [u8; 20] = hash[12..32].try_into().unwrap();
  format!("0x{}", hex::encode(trimmed))
}

fn format_secret_key(skey : SecretKey) -> String {
  hex::encode(skey.serialize())
}
