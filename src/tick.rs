use serde::{Deserialize, Serialize};

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
    pub time: i64, // 时间戳
    pub open: f64, // 开盘
    pub high: f64, // 最高
    pub low: f64,  // 最低
    pub close: f64, // 收盘
    pub spread: f64, // 点差
}
// 开盘 -> 最低 -> 最高 -> 收盘
