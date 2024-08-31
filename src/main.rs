use rand::Rng;
use crate::broker::Broker;
use crate::manager::Manager;
use crate::positions::PositionsType;
use crate::strategy::{DefaultStrategy, Strategy};
use crate::tick::Tick;

mod broker;
mod tick;
mod conf;
mod strategy;
mod positions;
mod utils;
mod error;
mod manager;
mod statistics;

fn main() {
    let mut manager = Manager::new(10000.0, 500,
                                   "EURUSDc_M15.csv".to_string(),
                                   Box::new(Ea1Strategy::new(10, 0.01, 15)));

    manager.backtesting();
}

struct Ea1Strategy {
    step: i64, // 步长
    initial_volume: f64, // 初始volume
    interval_time: u64, // 间隔时间分

    high: f64, // 持仓最高盈利价格
    sink: f64,
    sink1: f64,
    sink2: f64,
}

impl Ea1Strategy {
    fn new(step: i64, initial_volume: f64, interval_time: u64) -> Self {
        Ea1Strategy {
            step,
            initial_volume,
            interval_time,
            high: 0.0,
            sink: 30.0,
            sink1: 30.0,
            sink2: 60.0,
        }
    }
    fn close_all(&mut self, tick: Tick, broker: &mut Broker) {
        if broker.get_position_num() == 0 {
            return;
        }
        let profit = broker.get_profit();

        let mut profitable_quantity = 0; // 盈利订单数量
        let last_time = broker.get_last_position().unwrap().open_time; // 最新的订单时间
        // 如果都盈利  时间超过1h就平仓
        broker.get_positions().unwrap().iter().for_each(|position| {
            if position.profit > 0.0 {
                profitable_quantity += 1;
            }
        });
        if profitable_quantity == broker.get_position_num() && tick.time - last_time > 3600 {
            self.high = 0.0;
            broker.close_all();
            return;
        }

        if self.high < profit {
            self.high = profit;
        }

        if profit > 2.0 {
            if (self.high - profit >= self.sink) {
                self.high = 0.0;
                broker.close_all();
                return;
            }

            if (self.high >= 80.0 && profit <= self.sink2) {
                self.high = 0.0;
                broker.close_all();
                return;
            }

            if (self.high >= 40.0 && profit <= self.sink1) {
                self.high = 0.0;
                broker.close_all();
                return;
            }
        }
    }
}

impl Strategy for Ea1Strategy {
    fn on_tick(&mut self, tick: Tick, broker: &mut Broker) {
        self.close_all(tick.clone(), broker);
        if broker.get_position_num() == 0 {
            let mut rng = rand::thread_rng();
            let random_number: u8 = rng.gen_range(0..=1);
            if random_number == 0 {
                broker.buy("EURUSD".to_string(), self.initial_volume, 0.0, 15.0, "".to_string());
            } else {
                broker.sell("EURUSD".to_string(), self.initial_volume, 0.0, 15.0, "".to_string());
            }
            return;
        }

        let last_position = broker.get_last_position().unwrap();
        // 时间
        if self.interval_time * 60 > tick.time - last_position.open_time {
            return;
        }
        // 价格间隔
        if (tick.ask - last_position.open_price) < self.step as f64 * 0.0001 {
            return;
        }

        // 如果是盈利的
        if broker.get_profit() > 0.5 {
            return;
        }

        // 亏损加仓
        let new_volume = last_position.volume + self.initial_volume;
        match last_position.position_type {
            PositionsType::Buy => {
                broker.buy("EURUSD".to_string(), new_volume * 10.0, 15.0, 0.0, "".to_string());
            }
            PositionsType::Sell => {
                broker.sell("EURUSD".to_string(), new_volume * 10.0, 15.0, 0.0, "".to_string());
            }
        }
    }
}