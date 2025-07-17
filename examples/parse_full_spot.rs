use std::{fs::File, io::{Read, Write}};

use serde_json::Value;


fn main() {
    let path = "abci_state.full.json";

    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let value: Value = serde_json::from_slice(&buffer).unwrap();

    let exchange_data = value["exchange"].as_object().unwrap();

    for key in exchange_data.keys() {
        println!("{}", key);
    }

    let spot_clearinghouse = exchange_data["spot_clearinghouse"].as_object().unwrap();


    for (key, value) in spot_clearinghouse.iter() {
        let path = format!("spot_clearinghouse/{}.json", key);
        let mut file = File::create(path).unwrap();
        file.write_all(serde_json::to_string_pretty(value).unwrap().as_bytes()).unwrap();
    }
}
