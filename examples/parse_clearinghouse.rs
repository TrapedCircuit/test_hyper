use std::{fs::File, io::Read};

use serde_json::Value;

fn main() -> Result<(), anyhow::Error> {
    let path = "data/clearinghouse.json";
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let value: Value = serde_json::from_slice(&buffer).unwrap();

    let user_states = &value["user_states"];
    let user_to_state = user_states["user_to_state"].as_array().unwrap();

    let oracle_px = 2955.8;
    let mut total_value = 0.0;
    let mut total_szi = 0.0;
    for user in user_to_state {
        let (addr, value): (String, Value) = serde_json::from_value(user.clone()).unwrap();

        if let Some(p) = value.get("p") {
            if let Some(p) = p.get("p") {
                let positions = p.as_array().unwrap();
                for pos in positions {
                    let (asset_idx, pos_data): (u64, Value) =
                        serde_json::from_value(pos.clone()).unwrap();

                    if asset_idx == 1 {
                        if let Some(s) = pos_data.get("s") {
                            let s = match s.as_i64() {
                                Some(s) => s as f64 / 10_f64.powf(4.0),
                                None => {
                                    println!("{}, {}", addr, s);
                                    continue;
                                }
                            };
                            if s > 0.0 {
                                total_szi += s;
                            }

                            let e = pos_data.get("e").ok_or(anyhow::anyhow!("no e"))?;
                            let e = e.as_i64().ok_or(anyhow::anyhow!("no e"))? as f64
                                / 10_f64.powf(6.0);

                            let f = pos_data
                                .get("f")
                                .and_then(|v| v.get("o").and_then(|v| v.as_f64()))
                                .unwrap_or(0.0) / 10_f64.powf(6.0);

                            if let Some(l) = pos_data.get("l") {
                                let leverage = match (l.get("C"), l.get("I")) {
                                    (Some(c), _) => c.as_i64().ok_or(anyhow::anyhow!("no c l"))?,
                                    (None, Some(i)) => i
                                        .get("l")
                                        .ok_or(anyhow::anyhow!("no i l"))?
                                        .as_i64()
                                        .ok_or(anyhow::anyhow!("no i l"))?,
                                    _ => {
                                        println!("{}, {}", addr, l);
                                        continue;
                                    }
                                };

                                let value =
                                    real_position_value(s, e, oracle_px, f, leverage as u8);
                                total_value += value;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("total_value: {}", total_value);
    println!("total_szi: {}", total_szi);

    Ok(())
}

fn calculate_unrealized_pnl(szi: f64, e: f64, oracle_px: f64) -> f64 {
    if szi < 0.0 {
        e - oracle_px * szi.abs()
    } else {
        oracle_px * szi.abs() - e
    }
}

fn real_position_value(
    szi: f64,
    e: f64,
    oracle_px: f64,
    cum_funding_since_open: f64,
    leverage: u8,
) -> f64 {
    let unrealized_pnl = calculate_unrealized_pnl(szi, e, oracle_px);
    let margin_used = e / leverage as f64;

    unrealized_pnl + margin_used + cum_funding_since_open
}

#[test]
fn test_real_position_value() {
    let szi = -0.0167;
    let e = 49.998130;
    let oracle_px = 2953.6;
    let cum_funding_since_open = -0.029;
    let leverage = 25;

    let value = real_position_value(szi, e, oracle_px, cum_funding_since_open, leverage);
}
