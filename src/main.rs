use crate::broker::Broker;
use crate::manager::Manager;
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
    let mut manager = Manager::new(10000.0, 500, "EURUSDc_M15.csv".to_string(), Box::new(DefaultStrategy{}));

    manager.backtesting();
}

struct Ea1Strategy {

}

impl Strategy for Ea1Strategy {
    fn on_tick(&self, tick: Tick, broker: &mut Broker) {

    }
}