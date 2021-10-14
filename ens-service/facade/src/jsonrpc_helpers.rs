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

pub const JSON_RPC: &'static str = "2.0";

#[derive(Debug)]
pub struct Request {
    pub jsonrpc: String,
    pub eth_provider_url: String,
    pub method: String,
    pub id: u64,
}

impl Request {
    pub fn new(eth_provider_url: String, method: String, id: u64) -> Self {
        Request {
            jsonrpc: String::from(JSON_RPC),
            eth_provider_url,
            method,
            id,
        }
    }

    pub fn format(&self, params: Vec<String>) -> Vec<String> {

        let data = format!("{{\"jsonrpc\":\"{}\",\"method\":\"{}\", \"params\":{:?}, \"id\":{}}}", self.jsonrpc, self.method, params, self.id);
        
        let args = vec![
            String::from("-s"),
            String::from("-X"),
            String::from("POST"),
            String::from("--data"),
            data,
            self.eth_provider_url.to_string()
        ];

        args 
    }

    pub fn format_call(&self, to: &String, params: &String) -> Vec<String> {

        let data = format!("{{\"jsonrpc\":\"{}\",\"method\":\"{}\", \"params\":[{{\"to\":\"{}\",\"data\":\"{}\"}},\"latest\"], \"id\":{}}}", self.jsonrpc, self.method, to, params, self.id);
        
        let args = vec![
            String::from("-s"),
            String::from("-X"),
            String::from("POST"),
            String::from("-H"),
            String::from("Content-Type: application/json"),
            String::from("--data"),
            data,
            self.eth_provider_url.to_string()
        ];

        args 
    }
}

