mod conversion;

use std::os::windows::ffi::OsStringExt;
use std::sync::{mpsc, Arc, Mutex, Once};
use std::thread;
use csv::Writer;
use serde::Serialize;
use lazy_static::lazy_static;

#[derive(Debug, Serialize, Clone)]
struct LogData {
    version: String,
    order_total: i64,
    volume: f64,
    profit: f64,
    time: i64,
}

lazy_static! {
    static ref GLOBAL_DATA: Mutex<PigItem> = Mutex::new(PigItem {
        key: "".to_string(),
        volume: 0.0,
        profit: 0.0,
        total: 0
    });

    // 使用 `sync_channel` 并将容量设置为 5000
 static ref CHANNEL: (mpsc::SyncSender<LogData>, Arc<Mutex<mpsc::Receiver<LogData>>>) = {
        let (tx, rx) = mpsc::sync_channel(5000);
        (tx, Arc::new(Mutex::new(rx)))
    };
}

static INIT: Once = Once::new();

#[derive(Debug, Serialize, Clone)]
struct PigItem {
    key: String,
    volume: f64,
    profit: f64,
    total: i64,
}

#[no_mangle]
pub extern "system" fn rs_log(
    version: *mut u16,
    order_total: i64,
    volume: f64,
    profit: f64,
    time: i64,
) {
    INIT.call_once(|| {
        start_csv_writer_thread();
        // start_http_consumer_threads(5); // 创建5个HTTP消费者线程
    });

    // 使用转换函数将 version 指针转换为 String
    let version = match conversion::wide_ptr_to_string(version) {
        Some(ver) => ver,
        None => {
            eprintln!("Failed to convert version from wide string.");
            return;
        }
    };

    let log_data = LogData {
        version,
        order_total,
        volume,
        profit,
        time,
    };

    let mut update_db = false;
    {
        let mut global_data = GLOBAL_DATA.lock().unwrap();
        let key = format!("{}-{}", order_total, volume);
        if global_data.key != key {
            global_data.key = key;
            global_data.volume = volume;
            global_data.profit = profit;
            global_data.total += 1;
            update_db = true;
        } else if (profit - global_data.profit).abs() > 5.00 {
            update_db = true;
        }
    }

    if update_db {
        if let Err(e) = CHANNEL.0.send(log_data) {
            eprintln!("Failed to send data to the channel: {}", e);
        }
    }
}


fn start_csv_writer_thread() {
    let rx = Arc::clone(&CHANNEL.1);
    thread::spawn(move || {
        let mut wtr = Writer::from_path("output.csv").unwrap();
        loop {
            let log_data = {
                let rx = rx.lock().unwrap();
                rx.recv()
            };
            match log_data {
                Ok(data) => {
                    if wtr.serialize(&data).is_err() {
                        eprintln!("Failed to write log data to CSV");
                    }
                    if wtr.flush().is_err() {
                        eprintln!("Failed to flush CSV writer");
                    }
                }
                Err(_) => break,
            }
        }
    });
}

// fn start_http_consumer_threads(thread_count: usize) {
//     for i in 1..=thread_count {
//         let rx = Arc::clone(&CHANNEL.1);
//         thread::spawn(move || {
//             loop {
//                 let log_data = {
//                     let rx = rx.lock().unwrap();
//                     rx.recv()
//                 };
//                 match log_data {
//                     Ok(data) => {
//                         if let Err(e) = send_http_request(&data) {
//                             eprintln!("Consumer {}: Failed to send HTTP request: {}", i, e);
//                         }
//                     }
//                     Err(_) => break,
//                 }
//             }
//         });
//     }
// }
//
// fn send_http_request(log_data: &LogData) -> Result<(), Box<dyn std::error::Error>> {
//     let response = ureq::post("http://127.0.0.1:8181/statistics")
//         .set("Content-Type", "application/json")
//         .send_json(serde_json::to_value(log_data)?)?;
//
//     if response.status() != 200 {
//         Err(format!("HTTP request failed with status: {}", response.status()).into())
//     } else {
//         println!("Successfully sent log data: {:?}", log_data);
//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use super::*;

    #[test]
    fn test_conversion() {
        // 模拟调用 rs_log 函数
        // let version = OsStringExt::from_wide(&['V' as u16, '1' as u16, '2' as u16, '3' as u16, 0u16])
        //     .as_ptr() as *mut crate::rs_log;
        // let version = OsString::from("V123");
        // rs_log(version, 10, 100.0, 200.0, 123456789);

        // 让主线程等待一段时间，以确保消费者处理消息
        // thread::sleep(std::time::Duration::from_secs(5));
    }
}