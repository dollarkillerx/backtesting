use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PositionsType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Positions {
    pub id: u64, // 订单id
    pub symbol: String, // 货币对名称
    pub open_price: f64, // 开仓价格
    pub close_price: f64, // 平仓价格
    pub open_time: u64, // 开仓时间
    pub close_time: u64, // 平仓时间
    pub position_type: PositionsType, // 持仓类型
    pub volume: f64, // 持仓量
    pub profit: f64, // 盈亏
    pub sl: f64, // 止损
    pub tp: f64, // 止盈
    pub comment: String, // 备注
}