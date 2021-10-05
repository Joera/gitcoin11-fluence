use crate::curl_request;
use chrono::Utc;
use marine_rs_sdk::marine;
use serde::{ Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;

use std::collections::BTreeMap;
use indexmap::IndexMap;

const IPFS_NODE_READ_URL: &str = "http://64.227.70.116:8080";
const IPFS_NODE_WRITE_URL: &str = "http://64.227.70.116:5001";

#[marine]
fn update_tree(tip: String, new_item: String) -> String {


    let mut service_data = IndexMap::new();
    service_data.insert(String::from("templates"), String::from("governance"));
    service_data.insert(String::from("protocols"), String::from("random-electionator-2"));

 //   let new_item = String::from("{\"againstVotes\":\"630000\",\"endBlock\":\"5249422\",\"forVotes\":\"370000\",\"id\":\"19\",\"proposal\":{\"description\":\"end the wars\",\"id\":\"19\"},\"votes\":[{\"id\":\"0x5aa88a6a1ad8f4a71dc5a4e5946239984f36e87f-19\",\"voter\":{\"delegatedVotes\":\"0\"},\"votes\":\"370000\"},{\"id\":\"0xa6831dd52b1ccfbcaa860109cbb4ed0acd4bfc68-19\",\"voter\":{\"delegatedVotes\":\"0\"},\"votes\":\"80000\"},{\"id\":\"0xe986d774de323749f82ca13471c8c580316c05fb-19\",\"voter\":{\"delegatedVotes\":\"0\"},\"votes\":\"50000\"},{\"id\":\"0xf0e304b7fe717834a165d313197974634cf491f7-19\",\"voter\":{\"delegatedVotes\":\"0\"},\"votes\":\"130000\"},{\"id\":\"0xf816bf1d588100b6cea06b12cce53fa81e81246a-19\",\"voter\":{\"delegatedVotes\":\"0\"},\"votes\":\"370000\"}]}");

    let new_object: serde_json::Value = serde_json::from_str(&new_item).unwrap();
    let new_dag_cid: String = create_dag(new_item);// .cid_string;
    let tree = build_tree(tip,service_data.clone());
    
   //  println!("{:?}", new_object);
    let slug = serde_json::to_string(&new_object["id"]).unwrap();
    
    rebuild_tree(tree, service_data.clone(), new_dag_cid, slug)

    
}


fn build_tree(tip: String, service_data: IndexMap<String,String>) -> Vec<String> { 

    let mut nodes: Vec<String> = vec!();
  
    let mut node: Option<String> = Some(get_dag(&tip));
    let unwrapped_node = node.unwrap();
    nodes.push(unwrapped_node.clone());

    let mut i = 0;
    while i < service_data.len() {
   //     println!("i dunno {:?}",service_data.get_index(i));
        node = get_child(Some(nodes[i].clone()),service_data.get_index(i).unwrap());
        if node.is_none() {
            break
        }
        nodes.push(node.unwrap().clone());
        i+= 1;
    }

    nodes
}

// , new_node_slug: String
fn rebuild_tree(tree: Vec<String>, service_data: IndexMap<String,String>, new_dag_cid: String, new_node_slug: String) -> String{

 
    let reversed_tree: Vec<String> = tree.into_iter().rev().collect();
    let mut paths = service_data.clone();
    paths.reverse();

    let mut cid = new_dag_cid;
  
    let mut i = 0;
    while i < reversed_tree.len() {

            let node = reversed_tree[i].clone();

            if i == 0 {

                let new_node = replace_or_insert_link(node,cid, String::from("items"),new_node_slug.clone());
                cid = create_dag(new_node);

            } else if i == 1 {
                let key_to_include = paths.get_index(i - 1).unwrap().0.clone();
                let slug_to_include = paths.get_index(i - 1).unwrap().1.clone();
                let new_node = replace_or_insert_link(node,cid, key_to_include, slug_to_include);
                cid = create_dag(new_node);

            } else if i == 2 {
                let key_to_include = paths.get_index(i - 1).unwrap().0.clone();
                let slug_to_include = paths.get_index(i - 1).unwrap().1.clone();
                let new_node = replace_or_insert_link(node,cid, key_to_include, slug_to_include);
                cid = create_dag(new_node);
            }

            i+= 1;
    }

    cid
}

fn get_child(node: Option<String>, path:(&String,&String) ) -> Option<String> {
    
    let mut next_node: Option<String> = None;

    let json : serde_json::Value = serde_json::from_str(&node.unwrap()).unwrap();
    let reference_property = &json[path.0];

    if reference_property.is_object() {
        let reference = &reference_property.as_object().unwrap()[path.1];
        let cid = reference["/"].as_str().unwrap();
        let child: String = get_dag(&cid);
        next_node = Some(child);

    // i am not using this at the moment
    } else if reference_property.is_array() {
        let references = &reference_property.as_array().unwrap().clone();
        for reference in references {

            if reference[path.1]["/"].as_str().is_none() { break; }
            let cid = reference[path.1]["/"].as_str().unwrap();
            let child: String = get_dag(&cid);
            let value = json!(path.1);
            if json!(child)["slug"] == value  {
                next_node = Some(child);
            }
        }
    }
    
    next_node
}

fn replace_or_insert_link(node: String, cid: String, key: String, slug: String ) -> String  {

    let mut new_node : serde_json::Value = serde_json::from_str(&node).unwrap();
    let mut link = BTreeMap::new();
    link.insert(String::from("/"), &cid);
    new_node[key][slug] = json!(link);
    serde_json::to_string(&new_node).unwrap()
}

#[marine]
fn get_dag(tip: &str) -> String {

        let url = format!("{}/api/v0/dag/get/{}", IPFS_NODE_READ_URL,tip);

        let curl_args = vec![
            String::from("-s"),
            String::from("-X"),
            String::from("GET"),
            url
        ];

    let response = curl_request(curl_args);
     
    String::from_utf8(response.stdout).unwrap()
}

#[marine]
fn create_dag(data: String ) -> String { 

    let url = format!("{}/api/v0/dag/put?format=cbor&input-enc=json&pin=true", IPFS_NODE_WRITE_URL);

    let data_string = format!("file={}",data);

    let curl_args = vec![
        String::from("-s"),
        String::from("-X"),
        String::from("POST"),
        String::from("-F"),
        data_string,
        url
    ];

    let response = curl_request(curl_args);
    let response = String::from_utf8(response.stdout).unwrap();
    let json : serde_json::Value = serde_json::from_str(&response).unwrap();
 //   println!("{:?}", json["Cid"]["/"].as_str());
    json["Cid"]["/"].as_str().unwrap().to_string()
}