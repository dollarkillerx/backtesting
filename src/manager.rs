use std::sync::{mpsc};
use crate::broker::Broker;
use crate::tick::{Tick, TickManager};
use std::sync::mpsc::{Receiver};
use crate::strategy::Strategy;

pub struct Manager {
    broker: Broker, // Broker
    tick_channel: Receiver<Tick>, // TickManager
    strategy: Box<dyn Strategy>, // Strategy
}

impl Manager {
    pub fn new(balance: f64, lever: u64, kline_csv_path: String, strategy: Box<dyn Strategy>) -> Self {
        let (tx, rx) = mpsc::sync_channel(5000);
        let mut tick_manager = TickManager::new(kline_csv_path);
        tick_manager.set_tick_channel(tx);

        // 开启一个线程
        std::thread::spawn(move || {
            tick_manager.run();
        });

        Manager {
            broker: Broker::new(balance, lever),
            tick_channel: rx,
            strategy,
        }
    }

    // 回测
    pub fn backtesting(&mut self) {
        println!("Starting backtesting...");
        while let Ok(tick) = self.tick_channel.recv() {
            self.broker.on_tick(tick.clone());
            self.strategy.on_tick(tick, &mut self.broker);
        }
    }
}