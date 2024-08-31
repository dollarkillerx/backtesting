use std::fs::File;
use std::sync::mpsc::Receiver;
use std::thread;
use csv::{ReaderBuilder, Writer};
use plotters::prelude::*;
use serde::{Deserialize, Serialize};
use crate::positions::Positions;

// 统计模块
pub struct StatisticsServer {
    state_log_channel: Option<Receiver<StateLog>>, // 状态日志通道
    history_order: Option<Receiver<Positions>>, // 历史订单
}

impl StatisticsServer {
    pub fn new(state_log_channel: Receiver<StateLog>, history_order: Receiver<Positions>) -> Self {
        StatisticsServer {
            state_log_channel: Some(state_log_channel),
            history_order: Some(history_order),
        }
    }

    pub fn run(&mut self) {
        // 历史订单写操作
        if let Some(history_order_channel) = self.history_order.take() {
            thread::spawn(move || {
                Self::positions_write(history_order_channel);
            });
        }

        // 获取state_log_channel的所有权
        if let Some(state_log_channel) = self.state_log_channel.take() {
            thread::spawn(move || {
                Self::state_log_write(state_log_channel);
            });
        }

        println!("Statistics server has started.");
    }

    fn positions_write(history_order: Receiver<Positions>) {
        // 创建一个 CSV writer
        let mut wtr = Writer::from_path("history_order.csv").unwrap();

        // 读取history order 写文件
        for position in history_order.iter() {
            // 假设 Positions 可以直接序列化为 CSV 行
            wtr.serialize(position).unwrap();
        }

        // 刷新并关闭 writer
        wtr.flush().unwrap();
    }

    fn state_log_write(state_log_channel: Receiver<StateLog>) {
        // 创建一个 CSV writer
        let mut wtr = Writer::from_path("state_log.csv").unwrap();

        // 初始状态
        let mut previous_log: Option<StateLog> = None;
        let threshold = 10.0; // 盈亏变化的阈值
        for log in state_log_channel.iter() {
            if let Some(ref prev_log) = previous_log {
                if (log.profit - prev_log.profit).abs() >= threshold {
                    // 盈亏变化超过阈值，存储日志
                    wtr.serialize(&log).unwrap();
                    previous_log = Some(log);
                }
            } else {
                // 第一个日志，直接存储
                wtr.serialize(&log).unwrap();
                previous_log = Some(log);
            }
        }

        // 刷新并关闭 writer
        wtr.flush().unwrap();
    }

    // 修改为读取"state_log.csv" 文件生成图片
    pub fn generate_chart() -> Result<(), Box<dyn std::error::Error>> {
        // 打开 CSV 文件
        let file = File::open("state_log.csv")?;
        let mut rdr = ReaderBuilder::new().from_reader(file);

        // 读取 CSV 文件并解析为 StateLog 向量
        let mut logs: Vec<StateLog> = Vec::new();
        for result in rdr.deserialize() {
            let record: StateLog = result?;
            logs.push(record);
        }

        // 检查是否有数据
        if logs.is_empty() {
            return Err("No data available in state_log.csv".into());
        }

        // 创建一个图表区域，指定输出文件和图表尺寸
        let root = BitMapBackend::new("profit_balance_chart.png", (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        // 生成一个图表
        let mut chart = ChartBuilder::on(&root)
            .caption("Balance and Profit + Balance over Time", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                logs.first().unwrap().time..logs.last().unwrap().time, // X轴时间范围
                0.0..(logs.iter().map(|log| log.balance + log.profit).fold(f64::MIN, f64::max)), // 动态 Y 轴范围
            )?;

        // 绘制网格
        chart.configure_mesh().draw()?;

        // 绘制 Balance 线
        chart.draw_series(LineSeries::new(
            logs.iter().map(|log| (log.time, log.balance)),
            &BLUE,
        ))?.label("Balance")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        // 绘制 Profit + Balance 线
        chart.draw_series(LineSeries::new(
            logs.iter().map(|log| (log.time, log.profit + log.balance)),
            &RED,
        ))?.label("Profit")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        // 添加图例
        chart.configure_series_labels().border_style(&BLACK).draw()?;

        // 将结果写入文件
        root.present()?;
        println!("Chart has been saved to 'profit_balance_chart.png'");

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateLog {
    // 持仓订单数量
    pub positions_total: u64,
    // volume
    pub volume_total: f64,
    // 盈亏
    pub profit: f64,
    // 余额
    pub balance: f64,
    // time
    pub time: u64,
}

#[cfg(test)]
mod test {
    use plotters::prelude::*;
    use std::error::Error;
    use std::sync::mpsc;
    use std::thread;
    use crate::positions::{Positions, PositionsType};
    use crate::statistics::{StateLog, StatisticsServer};

    #[test]
    fn gen_img() -> Result<(), Box<dyn Error>> {
        pub struct StateLog {
            pub time: u64,
            pub profit: f64,
            pub balance: f64,
        }

        // 示例数据
        let data = vec![
            StateLog { time: 1577836800, profit: 100.0, balance: 1000.0 },
            StateLog { time: 1577923200, profit: 150.0, balance: 1100.0 },
            StateLog { time: 1578009600, profit: 200.0, balance: 1200.0 },
            StateLog { time: 1578096000, profit: 50.0, balance: 1150.0 },
            StateLog { time: 1578182400, profit: -20.0, balance: 1000.0 },
            StateLog { time: 1578268800, profit: 30.0, balance: 1030.0 },
        ];

        // 创建一个图表区域，指定输出文件和图表尺寸
        let root = BitMapBackend::new("output.png", (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        // 生成一个图表
        let mut chart = ChartBuilder::on(&root)
            .caption("Profit and Balance over Time", ("sans-serif", 50).into_font())
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                data.first().unwrap().time..data.last().unwrap().time, // X轴时间范围
                -100.0..1200.0, // Y轴范围，可以根据你的数据调整
            )?;

        // 绘制网格
        chart.configure_mesh().draw()?;

        // 绘制 profit 线
        chart.draw_series(LineSeries::new(
            data.iter().map(|log| (log.time, log.profit)),
            &RED,
        ))?.label("Profit")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        // 绘制 balance 线
        chart.draw_series(LineSeries::new(
            data.iter().map(|log| (log.time, log.balance)),
            &BLUE,
        ))?.label("Balance")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        // 添加图例
        chart.configure_series_labels().border_style(&BLACK).draw()?;

        // 将结果写入文件
        root.present()?;
        println!("Chart has been saved to 'output.png'");

        Ok(())
    }

    #[test]
    fn test_statistics_server() {
        let (state_log_tx, state_log_rx) = mpsc::channel();
        let (history_order_tx, history_order_rx) = mpsc::channel();

        let mut server = StatisticsServer::new(state_log_rx, history_order_rx);

        thread::spawn(move || {
            server.run();
        });

        // 发送一些测试数据
        state_log_tx.send(StateLog {
            positions_total: 5,
            volume_total: 100.0,
            profit: 50.0,
            balance: 1000.0,
            time: 1577836800,
        }).unwrap();

        state_log_tx.send(StateLog {
            positions_total: 5,
            volume_total: 100.0,
            profit: 60.0,
            balance: 1010.0,
            time: 1577836810,
        }).unwrap();

        history_order_tx.send(Positions {
            // 填写 Positions 数据
            id: 0,
            symbol: "".to_string(),
            open_price: 0.0,
            close_price: 0.0,
            open_time: 0,
            close_time: 0,
            position_type: PositionsType::Buy,
            volume: 0.0,
            profit: 0.0,
            sl: 0.0,
            tp: 0.0,
            comment: "".to_string(),
        }).unwrap();

        // 可以继续发送更多数据...

        // 添加一些延迟，确保测试线程有时间完成
        thread::sleep(std::time::Duration::from_secs(2));
    }
}