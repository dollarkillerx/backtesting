use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::sync::mpsc::{SyncSender};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json;

enum PERIOD {
    PeriodM5,
    PeriodM15,
    PeriodM30,
    PeriodH1,
    PeriodH4,
    PeriodD1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tick {
    pub symbol: String, // 货币对名称
    pub ask: f64,  // 买入价格
    pub bid: f64,  // 卖出价格
    pub time: u64, // 时间戳
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KLine {
    pub time: u64, // 时间戳
    pub open: f64, // 开盘
    pub high: f64, // 最高
    pub low: f64,  // 最低
    pub close: f64, // 收盘
    pub spread: u64, // 点差
}

// 开盘 -> 最低 -> 最高 -> 收盘
fn generate_ticks_from_kline(symbol: &str, kline: &KLine, steps: usize) -> Vec<Tick> {
    let mut ticks = Vec::new();

    // 计算时间间隔
    let time_increment = 1; // 每个 tick 的时间增量 (可以根据需要调整)

    // 从开盘价到最低价
    for i in 0..steps {
        let price = kline.open + i as f64 * (kline.low - kline.open) / steps as f64;
        ticks.push(Tick {
            symbol: symbol.to_string(),
            ask: price,
            bid: price - kline.spread as f64 * 0.00001, // 根据点差调整 bid 价格
            time: kline.time + i as u64 * time_increment,
        });
    }

    // 从最低价到最高价
    for i in 0..steps {
        let price = kline.low + i as f64 * (kline.high - kline.low) / steps as f64;
        ticks.push(Tick {
            symbol: symbol.to_string(),
            ask: price,
            bid: price - kline.spread as f64 * 0.00001,
            time: kline.time + (steps + i) as u64 * time_increment,
        });
    }

    // 从最高价到收盘价
    for i in 0..steps {
        let price = kline.high - i as f64 * (kline.high - kline.close) / steps as f64;
        ticks.push(Tick {
            symbol: symbol.to_string(),
            ask: price,
            bid: price - kline.spread as f64 * 0.00001,
            time: kline.time + (2 * steps + i) as u64 * time_increment,
        });
    }

    ticks
}



pub struct TickManager {
    k_lines: Vec<KLine>,
    tick_channel: Option<SyncSender<Tick>>
}

impl TickManager {
    pub fn new(kline_path: String) -> Self {
        let mut ks = Vec::new();
        let format = "%Y.%m.%d %H:%M:%S";
        let file = File::open(&kline_path).unwrap();
        let reader = io::BufReader::new(file);
        for line in reader.lines().skip(1) {
            let line = line.unwrap(); // 处理可能的错误
            let vec: Vec<String> = line.split('\t')
                .map(|s| s.to_string())  // 将 &str 转换为 String
                .collect();
            let naive_date_time = NaiveDateTime::parse_from_str(format!("{} {}", vec[0], vec[1]).as_str(), format).unwrap();
            // 将 NaiveDateTime 转换为 Unix 时间戳
            let timestamp = naive_date_time.and_utc().timestamp();
            ks.push(KLine {
                time: timestamp as u64,
                open: vec[2].parse::<f64>().unwrap(),
                high: vec[3].parse::<f64>().unwrap(),
                low: vec[4].parse::<f64>().unwrap(),
                close: vec[5].parse::<f64>().unwrap(),
                spread: vec[8].parse::<u64>().unwrap(),
            });
        }
        TickManager {
            k_lines: ks,
            tick_channel: None
        }
    }

    pub fn set_tick_channel(&mut self, channel: SyncSender<Tick> ) {
        self.tick_channel = Some(channel);
    }

    pub fn run(&mut self)  {
        if self.k_lines.is_empty() || self.tick_channel.is_none() {
            return;
        }

        println!("{}: {} lines of kline data", "TickManager", self.k_lines.len());
        self.k_lines.iter().for_each(|kline| {
            // send 开盘 -> 最低 -> 最高 -> 收盘
            let ticks = generate_ticks_from_kline("EURUSD", kline, 5);
            ticks.iter().for_each(|tick| {
                match self.tick_channel.as_ref().unwrap().send(tick.clone()) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("tick send error: {}", e);
                        return;
                    }
                };
            });
        });

        // 清空 k_lines
        self.k_lines.clear();

        // 关闭通道
        self.tick_channel = None;
    }
}


#[cfg(test)]
mod tests {
    use std::io;
    use std::io::BufRead;
    use serde_json::json;
    use super::*;

    #[test]
    fn test_tick() {
        let tm = TickManager::new("EURUSDc_M15.csv".to_string());
        println!("{:?}", tm.k_lines);
    }

    #[test]
    fn read_csv_test() {
        let format = "%Y.%m.%d %H:%M:%S";
        let path = "EURUSDc_M15.csv";

        // 打开文件
        let file = File::open(&path).unwrap();

        // 使用 BufReader 包装文件，逐行读取
        let reader = io::BufReader::new(file);

        // 迭代文件的每一行，并逐行打印
        for line in reader.lines().skip(1) {
            let line = line.unwrap(); // 处理可能的错误
            println!("{}", line);
            let vec: Vec<String> = line.split('\t')
                .map(|s| s.to_string())  // 将 &str 转换为 String
                .collect();
            println!("{:?}    len: {}", vec, vec.len());
            let naive_date_time = NaiveDateTime::parse_from_str(format!("{} {}", vec[0], vec[1]).as_str(), format).unwrap();

            // 将 NaiveDateTime 转换为 Unix 时间戳
            let timestamp = naive_date_time.and_utc().timestamp();

            let k_line = KLine {
                time: timestamp as u64,
                open: vec[2].parse::<f64>().unwrap(),
                high: vec[3].parse::<f64>().unwrap(),
                low: vec[4].parse::<f64>().unwrap(),
                close: vec[5].parse::<f64>().unwrap(),
                spread: vec[8].parse::<u64>().unwrap(),
            };

            println!("{:?}", k_line);
            return;
        }
    }

    #[test]
    fn test_generate_ticks_from_kline() {
        let kline = KLine {
            time: 1716300900,
            open: 1.08641,
            high: 1.0865,
            low: 1.08635,
            close: 1.08645,
            spread: 15,
        };

        let ticks = generate_ticks_from_kline("EURUSD", &kline, 10);

        println!("{}",  ticks.len());
        println!("{}", json!(ticks).to_string());

        // 验证生成的 tick 数量
        assert_eq!(ticks.len(), 9); // 3 (open->low) + 3 (low->high) + 3 (high->close)

        // 验证第一个 tick (open -> low)
        assert_eq!(ticks[0].ask, 1.08641);
        assert_eq!(ticks[0].bid, 1.08641 - 0.00015);
        assert_eq!(ticks[0].time, 1716300900);

        // 验证中间的 tick (low -> high)
        assert_eq!(ticks[4].ask, 1.086425);
        assert_eq!(ticks[4].bid, 1.086425 - 0.00015);
        assert_eq!(ticks[4].time, 1716300904);

        // 验证最后一个 tick (high -> close)
        assert_eq!(ticks[8].ask, 1.08645);
        assert_eq!(ticks[8].bid, 1.08645 - 0.00015);
        assert_eq!(ticks[8].time, 1716300908);
    }
}