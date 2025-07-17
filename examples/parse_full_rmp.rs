use std::{fs::File, io::Read};

use rmpv::Value;



fn main() {
    let path = "662460000.rmp";
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut cursor = std::io::Cursor::new(&buffer);
    let value: Value = rmpv::decode::read_value(&mut cursor).unwrap();

}
