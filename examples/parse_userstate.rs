use std::{
    fs::File,
    io::{Read, Write},
};

use serde_json::Value;

fn main() {
    let path = "clearinghouse/user_states.json";
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let value: Value = serde_json::from_slice(&buffer).unwrap();

    for (key, value) in value.as_object().unwrap().iter() {
        let file_name = format!("user_states/{}.json", key);

        let mut file = File::create(file_name).unwrap();
        file.write_all(serde_json::to_string_pretty(value).unwrap().as_bytes())
            .unwrap();
    }
}
