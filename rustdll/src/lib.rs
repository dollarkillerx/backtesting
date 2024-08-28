mod conversion;

use std::cmp::PartialEq;
use std::collections::HashSet;
use std::sync::Mutex;
use serde::Serialize;
use lazy_static::lazy_static;
use rand::Rng;

lazy_static! {
    static ref GLOBAL_DATA: Mutex<Vec<Order>> = Mutex::new(Vec::new());
    static ref GLOBAL_HIGH: Mutex<f64> = Mutex::new(0.0);

    // 用于生成唯一的随机数
    static ref UNIQUE_NUMBERS: Mutex<HashSet<i64>> = Mutex::new(HashSet::new());
}


fn get_latest_order() -> Option<Order> {
    let mut orders = GLOBAL_DATA.lock().unwrap();

    orders.sort_by(|a, b| b.time.cmp(&a.time)); // 按时间倒序排序

    orders.first().cloned() // 获取第一个元素，克隆到返回值中
}

fn remove_order_by_id(order_id: i64) -> bool {
    let mut orders = GLOBAL_DATA.lock().unwrap();
    if let Some(pos) = orders.iter().position(|order| order.id == order_id) {
        orders.remove(pos);
        true
    } else {
        false
    }
}

// 生成一个唯一的随机数
fn generate_unique_random_number(max_value: i64) -> i64 {
    let mut rng = rand::thread_rng();
    let mut unique_numbers = UNIQUE_NUMBERS.lock().unwrap();

    loop {
        let num = rng.gen_range(1..max_value);
        if unique_numbers.insert(num) {
            return num;
        }
    }
}

// 内存虚拟订单
// 开仓
// 加仓
// 出场

#[derive(PartialEq, Serialize, Debug, Clone)]
enum PositionType {
    Buy,
    Sell,
}

#[derive(Serialize, Debug, Clone)]
struct Order {
    id: i64,
    price: f64,
    volume: f64,
    sl: f64, // 止损
    tp: f64, // 止盈
    order_type: PositionType,
    time: i64,
}

#[no_mangle]  // is_buy 1 买 0 卖
pub extern "system" fn initial_order(price: f64,init_volume: f64, is_buy: i64, time: i64) -> i64{
    // 返回id
    // 清cache
    let mut orders = GLOBAL_DATA.lock().unwrap();
    orders.clear();
    let id = generate_unique_random_number(100000);
    orders.push({
        Order {
            id,
            price,
            time,
            volume: init_volume,
            sl: 0.0,
            tp: price + 0.0001 * 15.0,
            order_type: if is_buy == 1 { PositionType::Buy } else { PositionType::Sell },
        }
    });

    id
}

// 加仓
#[no_mangle]  // 如果加则放回 id 如果不加则返回 0   interval_time分
pub extern "system" fn add_order(ask_price: f64, bid_price: f64, time: i64,step: f64,interval_time: i64,initial_vol: f64 ) -> i64 {
    // 1. 获取上一个订单的基础信息
    let order = get_latest_order().unwrap();

    // 加仓逻辑
    // 1. 价格必须大于 step 步长
    if (ask_price - order.price).abs() < step {
        return 0;
    }
    // 2. 时间必须大于 interval_time 间隔时间
    if interval_time * 60 > time - order.time {
        return 0;
    }

    // 统计盈亏
    let mut profit = 0.0;
    {
        let orders = GLOBAL_DATA.lock().unwrap();
        for order in orders.iter() {
            match order.order_type {
                PositionType::Buy => {
                    profit += (ask_price - order.price) * order.volume;
                }
                PositionType::Sell => {
                    profit += (order.price - ask_price) * order.volume;
                }
            }
        }
    }

    if profit > 1.00 {
        return 0;
    }

    let new_vol = order.volume + initial_vol;
    let id = generate_unique_random_number(100000);
    if order.order_type == PositionType::Buy {
        {
            let mut orders = GLOBAL_DATA.lock().unwrap();
            orders.push({
                Order {
                    id,
                    price: ask_price,
                    time,
                    volume: new_vol,
                    sl: ask_price + 0.0001 * 15.0,
                    tp: 0.0,
                    order_type: PositionType::Buy,
                }});
        }
    }else{
        {
            let mut orders = GLOBAL_DATA.lock().unwrap();
            orders.push({
                Order {
                    id,
                    price: bid_price,
                    time,
                    volume: new_vol,
                    sl:bid_price + 0.0001 * 15.0,
                    tp: 0.0,
                    order_type: PositionType::Sell,
                }});
        }
    }

    id
}

#[no_mangle]
pub extern  "system" fn get_order_position_type(id: i64) -> i64 {
    // 1: buy 0: sell
    let orders = GLOBAL_DATA.lock().unwrap();
    for order in orders.iter() {
        if order.id == id {
            return if order.order_type == PositionType::Buy { 1 } else { 0 };
        }
    }

    // 未找到
    0
}

#[no_mangle]
pub extern  "system" fn get_order_volume(id: i64) -> f64 {
    // 1: buy 0: sell
    let orders = GLOBAL_DATA.lock().unwrap();
    for order in orders.iter() {
        if order.id == id {
            return order.volume;
        }
    }

    // 未找到
    0.00
}

#[no_mangle]
pub extern "system" fn auto_close(ask_price: f64, bid_price: f64) -> i64 {
    // 查看有没有止损或者止盈的订单
    // 如果有就平掉
    let orders = GLOBAL_DATA.lock().unwrap();
    for order in orders.iter() {
        match order.order_type {
            PositionType::Buy => {
                if order.sl != 0.0 {
                    if order.sl >= bid_price {
                        // close order
                        remove_order_by_id(order.id);
                        return order.id;
                    }
                }
                if order.tp != 0.0 {
                    if order.tp <= bid_price {
                        remove_order_by_id(order.id);
                        return order.id;
                    }
                }
            }
            PositionType::Sell => {
                if order.sl != 0.0 {
                    if order.sl <= ask_price {
                        remove_order_by_id(order.id);
                        return order.id;
                    }
                }
                if order.tp != 0.0 {
                    if order.tp >= ask_price {
                        remove_order_by_id(order.id);
                        return order.id;
                    }
                }
            }
        }
    }

    0
}


#[no_mangle]
pub extern "system" fn close_all(ask_price: f64, bid_price: f64, sink: f64, sink1: f64, sink2: f64) -> bool {
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
