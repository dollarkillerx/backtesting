use crate::broker::Broker;
use crate::manager::Manager;
use crate::strategy::DefaultStrategy;

mod broker;
mod tick;
mod conf;
mod strategy;
mod positions;
mod utils;
mod error;
mod manager;

fn main() {
    let mut manager = Manager::new(10000.0, 100, "EURUSDc_M15.csv".to_string(), Box::new(DefaultStrategy{}));

    manager.backtesting();
}
