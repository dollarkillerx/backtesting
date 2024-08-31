use std::sync::{mpsc};
use crate::broker::Broker;
use crate::tick::{Tick, TickManager};
use std::sync::mpsc::{Receiver};
use crate::statistics::StatisticsServer;
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

        // tick_manager线程
        std::thread::spawn(move || {
            tick_manager.run();
        });

        // statistics_server线程
        let (state_log_channel_tx, state_log_channel_rx) = mpsc::sync_channel(5000);
        let (history_order_tx, history_order_rx) = mpsc::sync_channel(5000);
        let mut statistics_server = StatisticsServer::new(state_log_channel_rx, history_order_rx);
        statistics_server.run();

        Manager {
            broker: Broker::new(balance, lever,state_log_channel_tx, history_order_tx),
            tick_channel: rx,
            strategy,
        }
    }

    // 回测
    pub fn backtesting(&mut self) {
        println!("Starting backtesting...");
        while let Ok(tick) = self.tick_channel.recv() {
            if self.broker.get_balance() <= 0.0 || self.broker.get_close_broker() {
                break;
            }
            self.broker.on_tick(tick.clone());
            self.strategy.on_tick(tick, &mut self.broker);
        }

        // 交易结束
        println!("Finished backtesting.");
        println!("Final balance: {}", self.broker.get_balance());
        println!("Final profit: {}", self.broker.get_profit());

        // 绘制图表
        StatisticsServer::generate_chart().unwrap();
    }
}