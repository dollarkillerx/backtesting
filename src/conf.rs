use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    symbol: String,
    m5_csv_path: String,
    m15_csv_path: String,
    m30_csv_path: String,
    h1_csv_path: String,
    h4_csv_path: String,
    d1_csv_path: String,
}

impl Config {
    pub fn new() -> Self {
        // 读取 TOML 文件
        let config_contents = fs::read_to_string("config.toml").unwrap();

        // 解析 TOML
        let config: Config = toml::from_str(&config_contents).unwrap();
        config
    }
}
