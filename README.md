# backtesting

好了感觉还是要自己写 基于rust的外汇回测框架

```
struct Manager {
    broker: Mutex<Broker>, // Broker                         经纪商模块 
    tick_channel: Mutex<Receiver<Tick>>, // TickManager      tick行情模块
    strategy: Box<dyn Strategy>, // Strategy                 策略模块
}
```

- 单货币 多周期回测
### 三方依赖
``` 
https://crates.io/crates/ta
https://crates.io/crates/polars
https://crates.io/crates/plotters
```

