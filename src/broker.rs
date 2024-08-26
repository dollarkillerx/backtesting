use std::collections::HashMap;
use crate::tick::Tick;

trait Strategy {
    fn on_tick(&self, tick: Tick);
}

struct DefaultStrategy {}

impl Strategy for DefaultStrategy {
    fn on_tick(&self, tick: Tick) {
        println!("on_tick: {:?}", tick);
    }
}

pub struct Broker {
    strategy: Box<dyn Strategy>,
    history_data: HashMap<String, HashMap<PERIOD, KLine>>
}

enum PERIOD {
    PeriodM5,
    PeriodM15,
    PeriodM30,
    PeriodH1,
    PeriodH4,
    PeriodD1,
}

struct KLine {
    date: u64, // 时间戳
    open: f64, // 开盘
    high: f64, // 最高
    low: f64, // 最低
    close: f64, // 收盘
    spread: f64, // 点差
}

impl Broker {
    fn new(strategy: Box<dyn Strategy>) -> Self {
        Self {
            strategy
        }
    }
    fn on_tick(&self, tick: Tick) {
        self.strategy.on_tick(tick);
    }
}

#[cfg(test)]
mod tests {
    use crate::broker::{Broker, DefaultStrategy};
    use crate::tick::Tick;

    #[test]
    fn it_works() {
        let broker = Broker::new(Box::new(DefaultStrategy {}));
        broker.on_tick(Tick { symbol: "btcusdt".to_string(), ask: 0.0, bid: 0.0, time: 0 });
    }
}