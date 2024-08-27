mod conversion;
use std::os::windows::ffi::OsStringExt;
use serde::Serialize;
use ureq;

#[derive(Debug, Serialize, Clone)]
struct LogData {
    version: String,
    order_total: i64,
    volume: f64,
    profit: f64,
    time: i64,
}

#[no_mangle]
pub extern "system" fn rs_log(
    version: *mut u16,
    order_total: i64,
    volume: f64,
    profit: f64,
    time: i64,
) {
    let version = conversion::wide_ptr_to_string(version).unwrap();

    // Create the log data structure
    let log_data = LogData {
        version,
        order_total,
        volume,
        profit,
        time,
    };

    // Serialize the structure into JSON
    let json_data = serde_json::to_string(&log_data).unwrap();

    // Send the POST request with the JSON data
    let response = ureq::post("http://127.0.0.1:8181/statistics")
        .set("Content-Type", "application/json")
        .send_string(&json_data);

    // Handle the response
    match response {
        Ok(resp) => {
            let body = resp.into_string().unwrap();
            println!("Response: {}", body);
        }
        Err(e) => {
            println!("Failed to send request: {}", e);
        }
    }
}