use std::fs;
use std::io::Read;
use rmpv::Value;
use serde_json;
use base64::{Engine as _, engine::general_purpose};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取rmp文件
    let file_path = "./656481200.rmp";

    let mut file = fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // 使用rmpv直接解析MessagePack到Value
    let mut cursor = std::io::Cursor::new(&buffer);
    let value: Value = rmpv::decode::read_value(&mut cursor)?;

    // 打印解析的结果

    // 转换为JSON格式
    let json_value = rmpv_to_json(&value)?;

    // 写入JSON文件
    let json_output = serde_json::to_string_pretty(&json_value)?;
    fs::write("656481200.json", json_output)?;
    println!("\n已将内容写入到 656481200.json 文件");

    Ok(())
}

// 将rmpv::Value转换为serde_json::Value
fn rmpv_to_json(value: &Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_value = match value {
        Value::Nil => serde_json::Value::Null,
        Value::Boolean(b) => serde_json::Value::Bool(*b),
        Value::Integer(i) => {
            if i.is_u64() {
                serde_json::Value::Number(serde_json::Number::from(i.as_u64().unwrap()))
            } else if i.is_i64() {
                serde_json::Value::Number(serde_json::Number::from(i.as_i64().unwrap()))
            } else {
                // 处理超大整数的情况
                serde_json::Value::String(i.to_string())
            }
        }
        Value::F32(f) => serde_json::Value::Number(
            serde_json::Number::from_f64(*f as f64)
                .unwrap_or_else(|| serde_json::Number::from(0))
        ),
        Value::F64(f) => serde_json::Value::Number(
            serde_json::Number::from_f64(*f)
                .unwrap_or_else(|| serde_json::Number::from(0))
        ),
        Value::String(s) => serde_json::Value::String(
            s.as_str().ok_or("Invalid UTF-8 string")?.to_string()
        ),
        Value::Binary(b) => {
            // 将二进制数据转换为base64字符串
            serde_json::Value::String(general_purpose::STANDARD.encode(b))
        }
        Value::Array(arr) => {
            let mut json_arr = Vec::new();
            for item in arr {
                json_arr.push(rmpv_to_json(item)?);
            }
            serde_json::Value::Array(json_arr)
        }
        Value::Map(map) => {
            let mut json_obj = serde_json::Map::new();
            for (key, val) in map {
                let key_str = match key {
                    Value::String(s) => s.as_str().ok_or("Invalid UTF-8 key")?.to_string(),
                    _ => format!("{:?}", key), // 如果键不是字符串，转换为调试格式
                };
                json_obj.insert(key_str, rmpv_to_json(val)?);
            }
            serde_json::Value::Object(json_obj)
        }
        Value::Ext(ty, data) => {
            // 扩展类型转换为对象
            let mut obj = serde_json::Map::new();
            obj.insert("ext_type".to_string(), serde_json::Value::Number(serde_json::Number::from(*ty)));
            obj.insert("data".to_string(), serde_json::Value::String(general_purpose::STANDARD.encode(data)));
            serde_json::Value::Object(obj)
        }
    };
    Ok(json_value)
}

// 辅助函数：获取Value的类型名称
fn get_value_type(value: &Value) -> &'static str {
    match value {
        Value::Nil => "Nil",
        Value::Boolean(_) => "Boolean",
        Value::Integer(_) => "Integer",
        Value::F32(_) => "F32",
        Value::F64(_) => "F64",
        Value::String(_) => "String",
        Value::Binary(_) => "Binary",
        Value::Array(_) => "Array",
        Value::Map(_) => "Map",
        Value::Ext(_, _) => "Extension",
    }
}
