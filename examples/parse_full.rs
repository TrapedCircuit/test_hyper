use std::{fs::File, io::{Read, Write}};

use serde_json::Value;

fn main() {
    let path = "658850000.full.json";

    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let value: Value = serde_json::from_slice(&buffer).unwrap();

    let exchange_data = value["exchange"].as_object().unwrap();
    let perp_dexs = &exchange_data["perp_dexs"].as_array().unwrap()[0];


    let perp_dexs_obj = perp_dexs.as_object().unwrap();
    let books = &perp_dexs_obj["books"];
    let funding_tracker = &perp_dexs_obj["funding_tracker"];
    let twap_tracker = &perp_dexs_obj["twap_tracker"];
    let schema = &perp_dexs_obj["schema"];
    let clearinghouse = &perp_dexs_obj["clearinghouse"];


    for key in clearinghouse.as_object().unwrap().keys() {
        println!("{}", key);
    }

    // let mut file = File::create("data/books.json").unwrap();
    // file.write_all(serde_json::to_string_pretty(&books).unwrap().as_bytes()).unwrap();

    // let mut file = File::create("data/clearinghouse.json").unwrap();
    // file.write_all(serde_json::to_string_pretty(&clearinghouse).unwrap().as_bytes()).unwrap();

    // let mut file = File::create("data/funding_tracker.json").unwrap();
    // file.write_all(serde_json::to_string_pretty(&funding_tracker).unwrap().as_bytes()).unwrap();

    // let mut file = File::create("data/twap_tracker.json").unwrap();
    // file.write_all(serde_json::to_string_pretty(&twap_tracker).unwrap().as_bytes()).unwrap();

    // let mut file = File::create("data/schema.json").unwrap();
    // file.write_all(serde_json::to_string_pretty(&schema).unwrap().as_bytes()).unwrap();

}
