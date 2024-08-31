use crate::broker::Broker;
use crate::error::BrokerError;
use crate::tick::Tick;

pub trait Strategy {
    fn on_tick(&self, tick: Tick, broker: &mut Broker);
}

pub struct DefaultStrategy {}

impl Strategy for DefaultStrategy {
    fn on_tick(&self, tick: Tick, broker: &mut Broker) {
        match broker.buy("EURUSD".to_string(), 1.0, 0.0, 0.0, "Test Buy".to_string()) {
            Ok(id) => {
                println!("buy id: {}", id);
            }
            Err(e) => {
                if let Some(BrokerError::InsufficientMargin) = e.downcast_ref::<BrokerError>() {
                    println!("Insufficient margin to buy {}  {}", broker.get_balance(), broker.get_profit());
                    if broker.get_position_num() == 0 {
                        broker.close_broker();
                    }
                    return;
                }
                println!("buy error: {}", e);
            }
        }
    }
}