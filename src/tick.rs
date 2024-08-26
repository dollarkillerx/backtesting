
#[derive(Debug, Clone)]
pub struct Tick {
    pub symbol: String, // 货币对名称
    pub ask: f64,  // 买入价格
    pub bid: f64,  // 卖出价格
    pub time: u64, // 时间戳
}