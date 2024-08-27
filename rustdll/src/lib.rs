mod conversion;
use std::os::windows::ffi::OsStringExt;
use std::sync::{mpsc, Arc, Mutex, Once};
use std::thread;
use csv::Writer;
use serde::Serialize;
use ureq;
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
    // 使用 `Once` 确保以下代码只执行一次
    INIT.call_once(|| {
        let rx = Arc::clone(&CHANNEL.1);
        // 创建一个 CSV writer
        let mut wtr = Writer::from_path("output.csv").unwrap();
        thread::spawn(move || {
            loop {
                let message = {
                    let rx = rx.lock().unwrap();
                    rx.recv()
                };

                match message {
                    Ok(message) => {
                       // write csv
                        wtr.serialize(message).unwrap();
                        wtr.flush().unwrap();
                    }
                    Err(_) => {
                        println!("Consumer : Channel closed");
                        break;
                    }
                }
            }
        });

        // 创建5个消费者线程
        // for i in 1..=5 {
        //     let rx = Arc::clone(&CHANNEL.1);
        //     thread::spawn(move || {
        //         loop {
        //             let message = {
        //                 let rx = rx.lock().unwrap();
        //                 rx.recv()
        //             };
        //
        //             match message {
        //                 Ok(message) => {
        //                     // Send the POST request with the JSON data
        //                     let response = ureq::post("http://127.0.0.1:8181/statistics")
        //                         .set("Content-Type", "application/json")
        //                         .send_string(&message);
        //
        //                     // Handle the response
        //                     match response {
        //                         Ok(resp) => {
        //                             match resp.into_string() {
        //                                 Ok(body) => println!("Consumer {}: Response: {}", i, body),
        //                                 Err(e) => println!("Consumer {}: Failed to read response: {}", i, e),
        //                             }
        //                         }
        //                         Err(e) => {
        //                             println!("Consumer {}: Failed to send request: {}", i, e);
        //                         }
        //                     }
        //                 }
        //                 Err(_) => {
        //                     println!("Consumer {}: Channel closed", i);
        //                     break;
        //                 }
        //             }
        //         }
        //     });
        // }
    });

    // 使用转换函数将 version 指针转换为 String
    let version = match conversion::wide_ptr_to_string(version) {
        Some(ver) => ver,
        None => {
            eprintln!("Failed to convert version from wide string.");
            return;
        }
    };

    // 创建日志数据结构
    let log_data = LogData {
        version,
        order_total,
        volume,
        profit,
        time,
    };

    let mut update_db = false;
    // 更新全局数据
    {
        let mut global_data = GLOBAL_DATA.lock().unwrap();

        let key = format!("{}-{}", order_total, volume);
        if global_data.key != key {
            global_data.key = key;
            global_data.volume = volume;
            global_data.profit = profit;
            global_data.total += 1;
            update_db = true;
        }else{
            if (profit - global_data.profit).abs() > 5.00 {
                update_db = true;
            }
        }

    }

    if update_db {
        // 将结构序列化为 JSON
        if CHANNEL.0.send(log_data).is_err() {
            eprintln!("Failed to send data to the channel.");
        }
        // match serde_json::to_string(&log_data) {
        //     Ok(json_data) => {
        //     if CHANNEL.0.send(json_data).is_err() {
        //         eprintln!("Failed to send data to the channel.");
        //     }
        //     }
        //     Err(e) => {
        //         eprintln!("Failed to serialize log data: {}", e);
        //     }
        // }
    }
}

