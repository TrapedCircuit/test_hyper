use std::fs;
use serde::{Deserialize, Serialize};

// 对应test.json的Rust结构体定义
#[derive(Debug, Serialize, Deserialize)]
pub struct RootData(String, UserData);

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    #[serde(rename = "b")]
    pub balances: Vec<u64>,
    #[serde(rename = "u")]
    pub orders: Vec<OrderEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderEntry(u64, OrderData);

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderData {
    pub core: OrderCore,
    pub trigger: OrderTrigger,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCore {
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "S")]
    pub size: u32,
    #[serde(rename = "r")]
    pub reduce_only: bool,
    #[serde(rename = "R")]
    pub range: OrderRange,
    #[serde(rename = "d")]
    pub direction: String, // "stopMarket", "takeProfitMarket", etc.
    #[serde(rename = "c")]
    pub c: Option<serde_json::Value>, // nullable field
    #[serde(rename = "b")]
    pub b: Option<serde_json::Value>, // nullable field
    #[serde(rename = "s")]
    pub side: String, // "A" for ask/sell, "B" for bid/buy
    #[serde(rename = "l")]
    pub limit_price: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRange {
    #[serde(rename = "p")]
    pub previous: u64,
    #[serde(rename = "s")]
    pub successor: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderTrigger {
    #[serde(rename = "isMarket")]
    pub is_market: bool,
    #[serde(rename = "triggerPx")]
    pub trigger_price: Vec<u64>,
    pub tpsl: String, // "sl" for stop loss, "tp" for take profit
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取test.json文件
    let json_content = fs::read_to_string("test.json")?;

    // 反序列化为Rust结构体
    let root_data: RootData = serde_json::from_str(&json_content)?;

    println!("✅ 解析test.json成功!");
    println!("📍 用户地址: {}", root_data.0);
    println!("💰 余额: {:?}", root_data.1.balances);
    println!("📋 订单数量: {}", root_data.1.orders.len());

    // 遍历并显示订单详情
    for (i, order_entry) in root_data.1.orders.iter().enumerate() {
        println!("\n🔖 订单 {}: ID = {}", i + 1, order_entry.0);
        let core = &order_entry.1.core;
        let trigger = &order_entry.1.trigger;

        println!("  📊 核心信息:");
        println!("    ⏰ 时间戳: {}", core.timestamp);
        println!("    📏 大小: {}", core.size);
        println!("    🎯 方向: {}", core.direction);
        println!("    📈 边: {} (A=卖单, B=买单)", core.side);
        println!("    💵 限价: {:?}", core.limit_price);

        println!("  🎯 触发器信息:");
        println!("    🏪 是否市价: {}", trigger.is_market);
        println!("    💲 触发价格: {:?}", trigger.trigger_price);
        println!("    🔄 类型: {} (sl=止损, tp=止盈)", trigger.tpsl);
    }

    Ok(())
}
