use std::{fs::File, io::Read};

use serde_json::Value;

fn main() {
    let file_path = "658720000.json";


    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let value: Vec<(String, Value)> = serde_json::from_slice(&buffer).unwrap();

    let mut btc_value = Value::Null;
    for (key, value) in value {
        if key == "BTC" {
            btc_value = value;
        }
    }


    let btc_vec = btc_value.as_array().unwrap();

    let btc_vec0 = btc_vec[0].as_array().unwrap();
    let side0 = btc_vec0[0]["side"].as_str().unwrap();
    let btc_vec1 = btc_vec[1].as_array().unwrap();
    let side1 = btc_vec1[0]["side"].as_str().unwrap();

    println!("side0: {}", side0);
    println!("side1: {}", side1);


    let mut total_bid_value = 0.0;
    for (i, v) in btc_vec0.iter().enumerate() {
        let sz = v["sz"].as_str().unwrap().parse::<f64>().unwrap();
        let price = v["limitPx"].as_str().unwrap().parse::<f64>().unwrap();
        total_bid_value += sz * price;
    }

    let mut total_ask_value = 0.0;
    for (i, v) in btc_vec1.iter().enumerate() {
        let sz = v["sz"].as_str().unwrap().parse::<f64>().unwrap();
        let price = v["limitPx"].as_str().unwrap().parse::<f64>().unwrap();
        total_ask_value += sz * price;
    }

    println!("total_bid_value: {}", total_bid_value);
    println!("total_ask_value: {}", total_ask_value);

}
