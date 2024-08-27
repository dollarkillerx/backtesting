mod conversion;
use std::os::windows::ffi::OsStringExt;
use serde::Serialize;
use serde_json::json;
use ureq;

#[derive(Debug, Serialize, Clone)]
struct LogData {
    order_total: i64,
    volume: f64,
    profit: f64,
    time: i64,
}


#[no_mangle]
pub extern "system" fn rs_log(order_total: i64, volume: f64, profit: f64, time: i64) {
    let body: String = ureq::get(&format!("http://127.0.0.1:8181/{}", json!(LogData {order_total, volume, profit, time}).to_string()))
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    println!("{}", body);
}

