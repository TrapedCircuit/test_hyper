use std::{fs::File, io::Read};

use rmpv::Value;



fn main() {
    let path = "/home/ethan/hlnode/hyperliquid_data/abci_state.rmp";


    loop {
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let mut cursor = std::io::Cursor::new(&buffer);
        let value: Value = rmpv::decode::read_value(&mut cursor).unwrap();

        let exchange = find_value(&value, "exchange").unwrap();
        let ctx = find_value(exchange, "ctx").unwrap();


        let height = find_value(ctx, "height").unwrap().as_u64().unwrap();

        println!("height: {}", height);

        drop(value);
    }

}


pub fn find_value<'a>(map: &'a rmpv::Value, key: &str) -> Option<&'a rmpv::Value> {
    map.as_map().and_then(|m| m.iter().find(|(k, _)| k.as_str() == Some(key)).map(|(_, v)| v))
}
