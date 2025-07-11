use std::{fs::File, io::Read};

use serde_json::Value;


fn main() {
    let path = "data/clearinghouse.json";
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();


    let value: Value = serde_json::from_slice(&buffer).unwrap();

    let user_states = &value["user_states"];

    let user_to_state = user_states["user_to_state"].as_array().unwrap();

    let user_0 = &user_to_state[0];
    let (addr, value): (String, Value) = serde_json::from_value(user_0.clone()).unwrap();

    let mut total_e = 0;
    let mut total_s = 0;
    let mut total_neg_s = 0;
    let mut long_trades = 0;
    let mut short_trades = 0;
    for user in user_to_state {
        let (addr, value): (String, Value) = serde_json::from_value(user.clone()).unwrap();

        if let Some(p) = value.get("p") {
            if let Some(p) = p.get("p") {
                let positions = p.as_array().unwrap();
                for pos in positions {
                    let (asset_idx, pos_data): (u64, Value) = serde_json::from_value(pos.clone()).unwrap();

                    if asset_idx == 1 {
                        if let Some(s) = pos_data.get("s") {
                            let s = s.as_i64().unwrap();

                            if s < 0 {
                                total_neg_s += s;
                                short_trades += 1;
                            } else {
                                total_s += s;
                                long_trades += 1;
                            }
                        }
                        if let Some(e) = pos_data.get("e") {
                            let e = e.as_i64().unwrap();
                            total_e += e;
                        }
                    }
                }
            }
        }
    }

    println!("total_s: {}", total_s);
    println!("total_neg_s: {}", total_neg_s);
    println!("total_e: {}", total_e);

    println!("long_trades: {}", long_trades);
    println!("short_trades: {}", short_trades);
}
