mod conversion;

use std::cmp::PartialEq;
use std::sync::Mutex;
use serde::Serialize;
use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBAL_DATA: Mutex<Vec<Order>> = Mutex::new(Vec::new());
    static ref GLOBAL_HIGH: Mutex<f64> = Mutex::new(0.0);
}

#[derive(PartialEq)]
enum PositionType {
    Buy,
    Sell,
}

struct Order {
    price: f64,
    volume: f64,
    order_type: PositionType,
}

#[no_mangle]
pub extern "system" fn buy(price: f64, volume: f64) {
    GLOBAL_DATA.lock().unwrap().push(Order {
        price,
        volume,
        order_type: PositionType::Buy
    });
}

#[no_mangle]
pub extern "system" fn sell(price: f64, volume: f64) {
    GLOBAL_DATA.lock().unwrap().push(Order {
        price,
        volume,
        order_type: PositionType::Sell
    });
}

#[no_mangle]
pub extern "system" fn ok_close(ask_price: f64, bid_price: f64, sink: f64, sink1: f64, sink2: f64) -> bool {
    // 统计所有订单当前盈亏
    let mut profit = 0.0;

    {
        let orders = GLOBAL_DATA.lock().unwrap();
        for order in orders.iter() {
            match order.order_type {
                PositionType::Buy => {
                    profit += (bid_price - order.price) * order.volume;
                }
                PositionType::Sell => {
                    profit += (order.price - ask_price) * order.volume;
                }
            }
        }
    }

    let mut high_value = {
        let mut high = GLOBAL_HIGH.lock().unwrap();
        if profit > *high {
            *high = profit;
        }
        *high
    };

    if high_value > 2.00 {
        if (high_value - profit >= sink) ||
            (high_value > 80.00 && profit <= sink2) ||
            (high_value > 40.00 && profit <= sink1) {
            let mut high = GLOBAL_HIGH.lock().unwrap();
            *high = 0.00;

            let mut orders = GLOBAL_DATA.lock().unwrap();
            orders.clear();

            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buy_sell_close() {
        buy(100.0, 1.0);
        sell(105.0, 1.0);

        // let closed = close(104.0, 2.0, 1.0, 0.5);
        // assert!(closed == false);
        //
        // let closed = close(103.0, 2.0, 1.0, 0.5);
        // assert!(closed == true);
    }
}
