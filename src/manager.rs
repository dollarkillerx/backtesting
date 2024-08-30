use std::sync::Mutex;
use crate::broker::Broker;

struct Manager {
    broker: Mutex<Broker> // Broker

}