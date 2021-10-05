use crate::curl_request;
// use chrono::Utc;
use marine_rs_sdk::marine;
use graphql_client::{ GraphQLQuery, Response };
// use serde::{Deserialize, Serialize};
use serde_json::json;

// call facade get_subgraph "19"

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",    // we hebben een probleem met BigInt, BigDecFDeal en Bytes
    query_path = "query.graphql",
    response_derives = "Debug,Serialize,PartialEq"
)]
struct ReferendumView;


#[marine]
pub fn get_subgraph(id: String) -> String {


    let url = String::from("https://api.studio.thegraph.com/query/4789/random-electionator-2/0.0.38");


    let variables = referendum_view::Variables {
        id: id
    };

    let request_body = ReferendumView::build_query(variables);
    let request_string = serde_json::to_string(&request_body).unwrap();
   
    let curl_args = vec![
            String::from("-s"),
            String::from("-X"),
            String::from("POST"),
            String::from("-H"),
            String::from("Content-Type: application/json"),
            String::from("--data"),
            request_string,
            url
        ];

    // println!("{:?}", curl_args);
    let response = curl_request(curl_args);
     
    let response = String::from_utf8(response.stdout).unwrap();
    let response_object: serde_json::Value = serde_json::from_str(&response).unwrap();
    let item_object = &response_object["data"]["referendum"];

    serde_json::to_string(&item_object).unwrap()

}