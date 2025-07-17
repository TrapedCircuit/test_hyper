use std::{fs::File, io::Read};

use serde_json::Value;

fn main() {
    let path = "data/books.json";
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let value: Value = serde_json::from_slice(&buffer).unwrap();

    let books = value.as_array().unwrap();

    let btc_book = books[0].as_object().unwrap();
    let asset = btc_book["asset"].as_i64().unwrap();
    println!("asset: {}", asset);
    let book_orders = btc_book["book_orders"].as_object().unwrap();

    for (key, value) in book_orders.iter() {
        let order = value.as_object().unwrap();

        // if let Some(r) = order.get("r") {
        //     if let Some(s) = order.get("c").and_then(|c| c.get("S")) {
        //         if s.as_u64().unwrap() != r.as_u64().unwrap() {
        //             println!("{}", value);
        //         }
        //     }
        // }
        if let Some(o) = order.get("O") {
            if let Some(s) = order.get("c").and_then(|c| c.get("S")) {
                if s.as_u64().unwrap() != o.as_u64().unwrap() {
                    println!("{}", value);
                }
            }
        }
    }

    println!("{}", book_orders.len());
}
