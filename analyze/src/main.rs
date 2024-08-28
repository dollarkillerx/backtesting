use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use chrono::NaiveDateTime;
use csv::Writer;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use statrs::statistics::Statistics;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AnaItem {
    total: i64,
    volume: f64,
    profile: f64,
    time: i64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "example_m30.csv";
    let data = load_data(file_path)?;


    let daily_profile_stats = calculate_daily_profile_stats(&data);

    write_to_csv("daily_profile_stats.csv", &daily_profile_stats)?;

    // 提取所有的 profile 值
    // let profiles: Vec<f64> = data.iter().map(|item| item.profile).collect();
    //
    // let (growth_count, loss_count) = count_growth_and_loss(&profiles);
    // println!("连续增长的次数: {}", growth_count);
    // println!("连续亏损的次数: {}", loss_count);
    //
    // if growth_count > loss_count {
    //     println!("增长次数多于亏损次数。");
    // } else if loss_count > growth_count {
    //     println!("亏损次数多于增长次数。");
    // } else {
    //     println!("增长和亏损次数相同。");
    // }


    // 计算平均数
    // let mean = (&profiles).mean();
    // println!("Profile 平均数: {}", mean);
    //
    // // 计算众数
    // let mode = calculate_mode(&profiles);
    // println!("Profile 众数: {}", mode);
    //
    // // 计算频率
    // // let frequency = calculate_frequency(&profiles);
    // // println!("Profile 频率: {:?}", frequency);
    //
    // // 计算 profile 与 volume 和 total 的相关性
    // let (volume_correlation, total_correlation) = calculate_correlations(&data);
    // println!("Profile 与 Volume 的相关性: {}", volume_correlation);
    // println!("Profile 与 Total 的相关性: {}", total_correlation);

    Ok(())
}


fn calculate_daily_profile_stats(data: &[AnaItem]) -> HashMap<String, (f64, f64)> {
    let mut daily_stats = HashMap::new();

    for item in data {
        // 将时间戳转换为日期
        let date = timestamp_to_date(item.time);

        // 获取当前日期的最低值和最高值
        let entry = daily_stats.entry(date).or_insert((item.profile, item.profile));

        // 更新最低值和最高值
        if item.profile < entry.0 {
            entry.0 = item.profile;
        }
        if item.profile > entry.1 {
            entry.1 = item.profile;
        }
    }

    daily_stats
}

fn timestamp_to_date(timestamp: i64) -> String {
    // 转换时间戳为 NaiveDateTime
    let datetime = NaiveDateTime::from_timestamp(timestamp, 0);
    // 提取日期部分
    let date = datetime.date();
    // 格式化日期为字符串
    date.format("%Y-%m-%d").to_string()
}

fn write_to_csv(file_path: &str, daily_profile_stats: &HashMap<String, (f64, f64)>) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;
    let mut wtr = Writer::from_writer(file);

    // 写入 CSV 文件的标题行
    wtr.write_record(&["Date", "Min Profile", "Max Profile"])?;

    // 写入数据
    for (date, (min_profile, max_profile)) in daily_profile_stats {
        wtr.write_record(&[date, &min_profile.to_string(), &max_profile.to_string()])?;
    }

    // 确保数据已写入
    wtr.flush()?;

    Ok(())
}

fn count_growth_and_loss(profiles: &[f64]) -> (usize, usize) {
    let mut growth_count = 0;
    let mut loss_count = 0;

    for i in 1..profiles.len() {
        if profiles[i] > profiles[i - 1] {
            growth_count += 1;
        } else if profiles[i] < profiles[i - 1] {
            loss_count += 1;
        }
    }

    (growth_count, loss_count)
}

fn load_data(file_path: &str) -> Result<Vec<AnaItem>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let data: Vec<AnaItem> = reader
        .lines()
        .skip(1) // 跳过第一行
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let fields: Vec<&str> = line.split(',').map(str::trim).collect();
            if fields.len() == 4 {
                Some(AnaItem {
                    total: fields[0].parse().ok()?,
                    volume: fields[1].parse().ok()?,
                    profile: fields[2].parse().ok()?,
                    time: fields[3].parse().ok()?,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(data)
}

fn calculate_mode(data: &[f64]) -> f64 {
    let mut occurrences = HashMap::new();
    let multiplier = 1000000.0; // 假设我们要保留 6 位小数

    for &value in data {
        let key = (value * multiplier).round() as i64;
        *occurrences.entry(key).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(key, _)| key as f64 / multiplier)
        .unwrap_or(0.0)
}

fn calculate_frequency(data: &[f64]) -> HashMap<OrderedFloat<f64>, usize> {
    let mut frequency_map = HashMap::new();

    for &value in data {
        *frequency_map.entry(OrderedFloat(value)).or_insert(0) += 1;
    }

    frequency_map
}

fn calculate_correlations(data: &[AnaItem]) -> (f64, f64) {
    let profiles: Vec<f64> = data.iter().map(|item| item.profile).collect();
    let volumes: Vec<f64> = data.iter().map(|item| item.volume).collect();
    let totals_f64: Vec<f64> = data.iter().map(|item| item.total as f64).collect();

    let volume_correlation = correlation(&profiles, &volumes);
    let total_correlation = correlation(&profiles, &totals_f64);

    (volume_correlation, total_correlation)
}

fn correlation(x: &[f64], y: &[f64]) -> f64 {
    let mean_x = x.mean();
    let mean_y = y.mean();

    let numerator: f64 = x.iter().zip(y.iter())
        .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
        .sum();

    let denominator_x: f64 = x.iter().map(|xi| (xi - mean_x).powi(2)).sum();
    let denominator_y: f64 = y.iter().map(|yi| (yi - mean_y).powi(2)).sum();

    numerator / (denominator_x * denominator_y).sqrt()
}

// 使用 BufReader 包装文件句柄
// let reader = io::BufReader::new(file);
//
// // 一行一行地读取文件
// for line in reader.lines() {
//     let line = line?; // line 是一个 Result<String, io::Error>
//     println!("{}", line);
// }

// 使用 has_headers(false) 来忽略 CSV 文件的标题行
// let mut rdr = ReaderBuilder::new()
//     .has_headers(false)
//     .delimiter(b',')
//     .from_reader(file);
//
// for result in rdr.deserialize() {
//     let record: AnaItem = result?;
//     println!("{:?}", record);
// }
