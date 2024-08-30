use std::fmt;

// 自定义错误类型
#[derive(Debug)]
pub enum BrokerError {
    InsufficientMargin,
    PositionNotFound,
}

impl fmt::Display for BrokerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrokerError::InsufficientMargin => write!(f, "保证金不足"),
            BrokerError::PositionNotFound => write!(f, "持仓不存在"),
        }
    }
}

impl std::error::Error for BrokerError {}
