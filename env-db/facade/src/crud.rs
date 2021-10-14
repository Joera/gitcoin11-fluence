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
use marine_rs_sdk::marine;
use marine_sqlite_connector;
use marine_sqlite_connector::{Connection, Value};

use crate::auth::is_owner;
use crate::get_connection;

pub fn create_table(conn: &Connection) -> std::result::Result<(), marine_sqlite_connector::Error> {
    let res = conn.execute(
        "
        create table if not exists pb_env (
            e text  not null primary key, 
            k text not null, 
        );

        ",
    );
    res
}

#[marine]
#[derive(Debug)]
pub struct UpdateResult {
    pub success: bool,
    pub err_str: String,
}

#[marine]
pub fn update_pb_env(data_string: String) -> UpdateResult {
    if !is_owner() {
        return UpdateResult {
            success: false,
            err_str: "You are not the owner".into(),
        };
    }

    let obj: serde_json::Value = serde_json::from_str(&data_string).unwrap();
    let obj = obj["result"].clone();

    let conn = get_connection();

    let insert = "insert or ignore into pb_env(?, ?, ?, ?)";
    let mut ins_cur = conn.prepare(insert).unwrap().cursor();

    let insert = ins_cur.bind(&[
        Value::String(obj["e"].to_string()),
        Value::String(obj["k"].to_string())
    ]);

    if insert.is_ok() {
        ins_cur.next().unwrap();
        let mut select = conn
            .prepare("select * from pb_env")
            .unwrap()
            .cursor();
        while let Some(row) = select.next().unwrap() {
            println!("select row {:?}", row);
            println!(
                "{}, {}",
                row[0].as_integer().unwrap(),
                row[2].as_string().unwrap()
            );
        }
        return UpdateResult {
            success: true,
            err_str: "".into(),
        };
    }

    UpdateResult {
        success: false,
        err_str: "Insert failed".into(),
    }
}

#[marine]
#[derive(Debug)]
pub struct PbEnv {
    pub e: String,
    pub k: String,
}

impl PbEnv {
    fn from_row(row: &[Value]) -> Self {
        PbEnv {
            e: row[0].as_string().unwrap().into(),
            k: row[1].as_integer().unwrap().to_string(),
        }
    }

    fn from_err() -> Self {
        PbEnv {
            e: String::from(""),
            k: String::from(""),
        }
    }
}

#[marine]
pub fn get_pb_env() -> PbEnv {
    // let db_path = "/tmp/db.sqlite";
    let conn = get_connection();
    let mut pb_env = PbEnv::from_err();

    let select = conn.prepare("select * from pb_env order by e desc limit 1");
    let result = match select {
        Ok(s) => {
            let mut select = s.cursor();
            while let Some(row) = select.next().unwrap() {
                println!("get_envs: {:?}", row);
                pb_env = PbEnv::from_row(row);
            }
            return pb_env;
        }
        Err(e) => pb_env,
    };
    result
}

