use crate::error::BrokerError;
use crate::positions::{Positions, PositionsType};
use crate::utils::generate_unique_id;

pub struct Broker {
    ask: f64, // 当前买价
    bid: f64, // 当前卖价
    time: u64, // 当前时间
    positions: Vec<Positions>, // 持仓
    history: Vec<Positions>, // 历史订单
    balance: f64, // 账户余额
    lever: u64, // 杠杆倍数
    profit: f64, // 盈亏
    margin: f64, // 保证金
}

impl Broker {
    pub fn new(balance: f64, lever: u64) -> Self {
        Broker {
            ask: 0.0,
            bid: 0.0,
            time: 0,
            positions: Vec::new(),
            history: Vec::new(),
            balance,
            lever,
            profit: 0.0,
            margin: 0.0,
        }
    }

    pub fn on_tick(&mut self, tick: crate::tick::Tick) {
        self.ask = tick.ask;
        self.bid = tick.bid;
        self.time = tick.time;

        // 自动平仓逻辑
        self.auto_close();
    }

    // 开单
    fn open_position(
        &mut self,
        symbol: String,
        position_type: PositionsType,
        volume: f64,
        sl: f64,
        tp: f64,
        comment: String,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        // 计算保证金是否足够
        let margin = (100000.0 * volume) / self.lever as f64;
        if self.balance - 50.0 <= margin {
            return Err(Box::new(BrokerError::InsufficientMargin));
        }

        let id = generate_unique_id();
        let open_price = match position_type {
            PositionsType::Buy => self.ask,
            PositionsType::Sell => self.bid,
        };
        let sl = match position_type {
            PositionsType::Buy => self.bid - 0.0001 * sl,
            PositionsType::Sell => self.ask + 0.0001 * sl,
        };
        let tp = match position_type {
            PositionsType::Buy => self.bid + 0.0001 * tp,
            PositionsType::Sell => self.ask - 0.0001 * tp,
        };

        let positions = Positions {
            id,
            symbol,
            open_price,
            close_price: 0.0,
            open_time: self.time,
            close_time: 0,
            position_type,
            volume,
            profit: 0.0,
            sl,
            tp,
            comment,
        };

        // 修改余额
        self.margin += margin;
        self.balance -= margin;

        // 插入订单
        self.positions.push(positions);
        println!("Orders: {:?}", self.positions);

        Ok(id)
    }

    pub fn buy(
        &mut self,
        symbol: String,
        volume: f64,
        sl: f64,
        tp: f64,
        comment: String,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        self.open_position(symbol, PositionsType::Buy, volume, sl, tp, comment)
    }

    pub fn sell(
        &mut self,
        symbol: String,
        volume: f64,
        sl: f64,
        tp: f64,
        comment: String,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        self.open_position(symbol, PositionsType::Sell, volume, sl, tp, comment)
    }

    // 平仓所有订单
    pub fn close_all(&mut self) {
        // 平仓并将订单移入历史订单中
        let mut closed_positions = Vec::new();

        for position in &mut self.positions {
            if position.close_time == 0 {
                position.close_time = self.time;
                let close_price = match position.position_type {
                    PositionsType::Buy => self.bid,
                    PositionsType::Sell => self.ask,
                };
                position.close_price = close_price;

                let profit = match position.position_type {
                    PositionsType::Buy => (close_price - position.open_price) * position.volume * 100000.0,
                    PositionsType::Sell => (position.open_price - close_price) * position.volume * 100000.0,
                };

                position.profit = profit;
                self.profit += profit;

                let margin = (100000.0 * position.volume) / self.lever as f64;
                self.margin -= margin;
                self.balance += margin + profit;

                closed_positions.push(position.clone());
            }
        }

        // 移除已平仓的订单
        self.positions.retain(|pos| pos.close_time == 0);

        // 将平仓的订单加入历史订单中
        self.history.extend(closed_positions);
    }

    // 自动平仓逻辑
    pub fn auto_close(&mut self) {
        let mut closed_positions = Vec::new();

        for position in &mut self.positions {
            let should_close = match position.position_type {
                PositionsType::Buy => {
                    (position.sl != 0.0 && position.sl >= self.bid) || (position.tp != 0.0 && position.tp <= self.bid)
                }
                PositionsType::Sell => {
                    (position.sl != 0.0 && position.sl <= self.ask) || (position.tp != 0.0 && position.tp >= self.ask)
                }
            };

            if should_close {
                position.close_time = self.time;
                let close_price = match position.position_type {
                    PositionsType::Buy => self.bid,
                    PositionsType::Sell => self.ask,
                };
                position.close_price = close_price;

                let profit = match position.position_type {
                    PositionsType::Buy => (close_price - position.open_price) * position.volume * 100000.0,
                    PositionsType::Sell => (position.open_price - close_price) * position.volume * 100000.0,
                };

                position.profit = profit;
                self.profit += profit;

                let margin = (100000.0 * position.volume) / self.lever as f64;
                self.margin -= margin;
                self.balance += margin + profit;
                closed_positions.push(position.clone());
                println!("AutoClose {} Price: {} Volume: {}", position.id, position.close_price, position.volume);
            }
        }

        // 移除已平仓的订单
        self.positions.retain(|pos| pos.close_time == 0);

        // 将平仓的订单加入历史订单中
        self.history.extend(closed_positions);
    }


    // 平仓指定订单
    pub fn close_position(&mut self, id: u64) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(index) = self.positions.iter().position(|pos| pos.id == id && pos.close_time == 0) {
            let mut position = self.positions[index].clone();

            position.close_time = self.time;
            let close_price = match position.position_type {
                PositionsType::Buy => self.bid,
                PositionsType::Sell => self.ask,
            };
            position.close_price = close_price;

            let profit = match position.position_type {
                PositionsType::Buy => (close_price - position.open_price) * position.volume * 100000.0,
                PositionsType::Sell => (position.open_price - close_price) * position.volume * 100000.0,
            };

            position.profit = profit;
            self.profit += profit;

            let margin = (100000.0 * position.volume) / self.lever as f64;
            self.margin -= margin;
            self.balance += margin + profit;

            // 将订单从持仓移动到历史订单
            self.history.push(position.clone());
            self.positions.remove(index);

            Ok(())
        } else {
            Err(Box::new(BrokerError::PositionNotFound))
        }
    }

    // 获取持仓订单数量
    pub fn get_position_num(&self) -> u64 {
        self.positions.len() as u64
    }

    // 获取当前盈亏
    pub fn get_profit(&mut self) -> f64 {
        let mut total_profit = 0.0;

        for position in &self.positions {
            let current_price = match position.position_type {
                PositionsType::Buy => self.bid,
                PositionsType::Sell => self.ask,
            };

            let profit = match position.position_type {
                PositionsType::Buy => (current_price - position.open_price) * position.volume * 100000.0,
                PositionsType::Sell => (position.open_price - current_price) * position.volume * 100000.0,
            };

            total_profit += profit;
        }

        self.profit = total_profit;
        self.profit
    }

    // 获取当前余额
    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    // 获取最后一个订单
    pub fn get_last_position(&self) -> Option<Positions> {
        self.positions.iter().max_by_key(|pos| pos.open_time).cloned()
    }

    // 获取当前持仓订单（按时间倒序排列）
    pub fn get_positions(&self) -> Option<Vec<Positions>> {
        if self.positions.is_empty() {
            None
        } else {
            let mut sorted_positions = self.positions.clone();
            sorted_positions.sort_by_key(|pos| std::cmp::Reverse(pos.open_time)); // 按时间倒序排列
            Some(sorted_positions)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::positions::{PositionsType, Positions};
    use crate::tick::Tick;

    fn create_test_broker() -> Broker {
        Broker::new(10000.0, 100)
    }

    fn create_test_tick(ask: f64, bid: f64, time: u64) -> Tick {
        Tick {
            symbol: "EURUSD".to_string(),
            ask,
            bid,
            time,
        }
    }

    #[test]
    fn test_open_position() {
        let mut broker = create_test_broker();
        broker.on_tick(create_test_tick(1.2, 1.1, 123456)); // 设置初始价格

        let result = broker.buy("EURUSD".to_string(), 1.0, 50.0, 100.0, "Test Buy".to_string());
        assert!(result.is_ok());
        assert_eq!(broker.get_position_num(), 1);
    }

    #[test]
    fn test_close_position() {
        let mut broker = create_test_broker();
        broker.on_tick(create_test_tick(1.2, 1.1, 123456)); // 设置初始价格

        let buy_id = broker.buy("EURUSD".to_string(), 1.0, 55.0, 120.0, "Test Buy".to_string()).unwrap();

        assert_eq!(broker.get_position_num(), 1);

        // 模拟价格跳动，使订单达到止盈或止损点
        broker.on_tick(create_test_tick(1.21, 1.11, 123457)); // 进一步上涨

        let close_result = broker.close_position(buy_id);
        assert!(close_result.is_ok());
        assert_eq!(broker.get_position_num(), 0);
        assert_eq!(broker.history.len(), 1);
    }

    #[test]
    fn test_get_profit() {
        let mut broker = create_test_broker();
        broker.on_tick(create_test_tick(1.2, 1.1, 123456)); // 设置初始价格

        broker.buy("EURUSD".to_string(), 1.0, 50.0, 100.0, "Test Buy".to_string()).unwrap();

        // 模拟价格跳动
        broker.on_tick(create_test_tick(1.3, 1.2, 123457)); // 价格上涨

        let profit = broker.get_profit();
        assert!(profit >= 0.0); // 根据价格变化，利润应该为正
    }

    #[test]
    fn test_auto_close() {
        let mut broker = create_test_broker();
        broker.on_tick(create_test_tick(1.2, 1.1, 123456)); // 设置初始价格

        broker.buy("EURUSD".to_string(), 1.0, 1.5, 1.5, "Test Buy".to_string()).unwrap();

        // 模拟价格跳动，触发止盈或止损
        broker.on_tick(create_test_tick(1.0, 0.5, 123457)); // 价格大幅下跌

        assert_eq!(broker.get_position_num(), 0);
        assert_eq!(broker.history.len(), 1);
    }

    #[test]
    fn test_get_last_position() {
        let mut broker = create_test_broker();
        broker.on_tick(create_test_tick(1.2, 1.1, 123456)); // 设置初始价格

        broker.buy("EURUSD".to_string(), 1.0, 50.0, 100.0, "First Buy".to_string()).unwrap();
        broker.on_tick(create_test_tick(1.25, 1.15, 123457)); // 价格变化

        broker.buy("GBPUSD".to_string(), 1.0, 50.0, 100.0, "Second Buy".to_string()).unwrap();
        broker.on_tick(create_test_tick(1.3, 1.2, 123458)); // 价格再次变化

        let last_position = broker.get_last_position();
        assert!(last_position.is_some());
        assert_eq!(last_position.unwrap().comment, "Second Buy");
    }

    #[test]
    fn test_get_positions_sorted() {
        let mut broker = create_test_broker();
        broker.on_tick(create_test_tick(1.2, 1.1, 123456)); // 设置初始价格

        broker.buy("EURUSD".to_string(), 1.0, 50.0, 100.0, "First Buy".to_string()).unwrap();
        broker.on_tick(create_test_tick(1.25, 1.15, 123457)); // 价格变化

        broker.buy("GBPUSD".to_string(), 1.0, 50.0, 100.0, "Second Buy".to_string()).unwrap();
        broker.on_tick(create_test_tick(1.3, 1.2, 123458)); // 价格再次变化

        let positions = broker.get_positions();
        assert!(positions.is_some());

        let positions = positions.unwrap();
        assert_eq!(positions[0].comment, "Second Buy");
        assert_eq!(positions[1].comment, "First Buy");
    }
}
