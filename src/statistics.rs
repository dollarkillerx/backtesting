use std::sync::mpsc::Receiver;
use crate::positions::Positions;

// 统计模块
pub struct Statistics {
    state_log_channel: Receiver<StateLog>, // 状态日志通道
    history_order: Receiver<Positions>, // 历史订单
}

pub struct StateLog {
    // 持仓订单数量
    positions_total: u64,
    // volume
    volume_total: f64,
    // 盈亏
    profit: f64,
    // 余额
    balance: f64,
    // time
    time: u64,
}


#[cfg(test)]
mod test {
    use plotters::prelude::*;
    use std::error::Error;
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
}