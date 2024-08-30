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