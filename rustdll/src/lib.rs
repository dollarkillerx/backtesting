mod conversion;
use std::cmp::PartialEq;
use rand::Rng;
use serde::{Deserialize, Serialize};
use ureq;

// 内存虚拟订单
// 开仓
// 加仓
// 出场

#[derive(Deserialize, Serialize, Debug, Clone)]
struct InitialOrderPayload {
    price: f64,
    init_volume: f64,
    is_buy: i64, // 止损
    time: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct OrderResponse {
    order_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct OrderTypeResponse {
    position_type: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct OrderVolumeResponse {
    volume: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct MarkPrice {
    ask: f64,
    bid: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct CloseAllPayload {
    ask: f64,
    bid: f64,
    time: i64,
    sink: f64,
    sink1: f64,
    sink2: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct CloseAllResponse {
    close_all: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddOrderPayload {
    ask: f64,
    bid: f64,
    time: i64,
    step: f64,
    interval_time: i64,
    initial_volume: f64,
}

const URL: &str = "http://127.0.0.1:8181";

#[no_mangle]
pub extern "system" fn initial_order(price: f64, init_volume: f64, is_buy: i64, time: i64) -> i64 {
    let order = InitialOrderPayload {
        price,
        time,
        init_volume,
        is_buy,
    };

    match ureq::post(&format!("{URL}/initial_order"))
        .set("Content-Type", "application/json")
        .send_json(serde_json::to_value(order).unwrap())
    {
        Ok(response) => {
            if let Ok(order_response) = response.into_json::<OrderResponse>() {
                order_response.order_id
            } else {
                -1 // JSON parsing error
            }
        }
        Err(_) => -1 // Network error or server error
    }
}

#[no_mangle]
pub extern "system" fn my_version() -> i64 {
    1
}

// 加仓
#[no_mangle]  // 如果加则放回 id 如果不加则返回 0   interval_time分
pub extern "system" fn add_order(ask: f64, bid: f64, time: i64, step: f64, interval_time: i64, initial_volume: f64) -> i64 {
    let payload = AddOrderPayload {
        ask,
        bid,
        time,
        step,
        interval_time: 10,
        initial_volume,
    };

    match ureq::post(&format!("{URL}/add_order"))
        .set("Content-Type", "application/json")
        .send_json(serde_json::to_value(payload).unwrap())
    {
        Ok(response) => {
            if let Ok(order_response) = response.into_json::<OrderResponse>() {
                order_response.order_id
            } else {
                -1 // JSON parsing error
            }
        }
        Err(_) => -1 // Network error or server error
    }
}

#[no_mangle]
pub extern "system" fn get_order_position_type(id: i64) -> i64 {
    let payload = OrderResponse {
        order_id: id,
    };

    match ureq::post(&format!("{URL}/get_order_position_type"))
        .set("Content-Type", "application/json")
        .send_json(serde_json::to_value(payload).unwrap())
    {
        Ok(response) => {

            if let Ok(order_response) = response.into_json::<OrderTypeResponse>() {
                order_response.position_type
            } else {
                -1 // JSON parsing error
            }
        }
        Err(_) => -1 // Network error or server error
    }
}

#[no_mangle]
pub extern "system" fn get_order_volume(id: i64) -> f64 {
    let payload = OrderResponse {
        order_id: id,
    };

    match ureq::post(&format!("{URL}/get_order_volume"))
        .set("Content-Type", "application/json")
        .send_json(serde_json::to_value(payload).unwrap())
    {
        Ok(response) => {
            if let Ok(order_response) = response.into_json::<OrderVolumeResponse>() {
                order_response.volume
            } else {
                -1.00 // JSON parsing error
            }
        }
        Err(_) => -1.00 // Network error or server error
    }
}

#[no_mangle]
pub extern "system" fn auto_close(ask: f64, bid: f64) -> i64 {
    // 查看有没有止损或者止盈的订单
    // 如果有就平掉
    let payload = MarkPrice {
        ask,
        bid,
    };

    let result = match ureq::post(&format!("{URL}/auto_close"))
        .set("Content-Type", "application/json")
        .send_json(serde_json::to_value(payload).unwrap_or_default())
    {
        Ok(response) => {
            // 将响应转换为字符串
            let raw_response = match response.into_string() {
                Ok(s) => s,
                Err(_) => {
                    if let Err(e) = ureq::get(&format!("{URL}/test/Failed to read response as string")).call() {
                        eprintln!("Failed to send test request: {}", e);
                    }
                    eprintln!("Failed to read response as string");
                    return -1;
                }
            };

            // 尝试解析 JSON
            match serde_json::from_str::<OrderResponse>(&raw_response) {
                Ok(order_response) => order_response.order_id,
                Err(e) => {
                    // 如果 JSON 解析失败，打印原始响应内容
                    eprintln!("JSON parsing error: {}. Raw response: {}", e, raw_response);
                    if let Err(e) = ureq::get(&format!("{URL}/test/json")).call() {
                        eprintln!("Failed to send test request: {}", e);
                    }
                    -1 // JSON parsing error
                }
            }
        }
        Err(e) => {
            eprintln!("Request error: {}", e);
            if let Err(e) = ureq::get(&format!("{URL}/test/er{}", e)).call() {
                eprintln!("Failed to send test request: {}", e);
            }
            -1 // Network error or server error
        }
    };

    if result == -1 {
        if let Err(e) = ureq::get(&format!("{URL}/test/what-1")).call() {
            eprintln!("Failed to send test request: {}", e);
        }
    }
    result
}


#[no_mangle]
pub extern "system" fn close_all(ask: f64, bid: f64, time: i64, sink: f64, sink1: f64, sink2: f64) -> bool {
    let payload = CloseAllPayload {
        ask,
        bid,
        time,
        sink,
        sink1,
        sink2,
    };

    match ureq::post(&format!("{URL}/close_all"))
        .set("Content-Type", "application/json")
        .send_json(serde_json::to_value(payload).unwrap())
    {
        Ok(response) => {
            if let Ok(order_response) = response.into_json::<CloseAllResponse>() {
                order_response.close_all
            } else {
                false // JSON parsing error
            }
        }
        Err(_) => false // Network error or server error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_latest_order() {
        let id = initial_order(10.00, 0.01, 1, 1724912590);
        println!("order id: {}", id);

        let p = add_order(20.5, 20.2, 1724962599, 10.00, 10, 0.01);
        println!("order id2: {}", p);

        let px = get_order_position_type(p);
        println!("position type: {}", px);

        let p = get_order_volume(p);
        println!("order volume: {}", p);

        let p = auto_close(20.5, 20.2);
        println!("auto_close: {}", p);

        let p = auto_close(20.5, 20.2);
        println!("auto_close: {}", p);

        let p = auto_close(20.5, 20.2);
        println!("auto_close: {}", p);
    }
}

