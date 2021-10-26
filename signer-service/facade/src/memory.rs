use std::cell::RefCell;
use std::thread::LocalKey;

use ethers_core::types::{ Address, PrivateKey };

thread_local!(
    static ADDRESS: RefCell<String> = RefCell::new(String::from(""));
    static KEY: RefCell<String> = RefCell::new(String::from(""));
);

pub fn store(address: &String, secret_string: &String) -> () {

    ADDRESS.with(|address_cell| {
      address_cell.replace(address.clone());
    });
  
    KEY.with(|key_cell| {
          key_cell.replace(secret_string.clone());
    });
  
} 

pub fn unitialized() -> bool {

    let mut key = String::from("");

    KEY.with(|key_cell| {        
        key = key_cell.borrow_mut().to_string()
    });

    if key == "" {
        true
    } else {
        false
    }
}

pub fn key() -> String {

    let mut key = String::from("");

    KEY.with(|key_cell| {        
        key = key_cell.borrow_mut().to_string()
    });

    key
}

pub fn address() -> String {

    let mut address = String::from("");

    ADDRESS.with(|address_cell| {
        address = address_cell.borrow_mut().to_string();
    });

    address
}