use crate::broker::Broker;
use crate::tick::Tick;

pub trait Strategy {
    fn on_tick(&self, tick: Tick, broker: &mut Broker);
}

pub struct DefaultStrategy {}

impl Strategy for DefaultStrategy {
    fn on_tick(&self, tick: Tick, broker: &mut Broker) {
        println!("DefaultStrategy on_tick: {:?}", tick);
    }
}