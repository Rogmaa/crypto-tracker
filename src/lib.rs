pub mod core;
pub mod infra;
pub mod presentation;

pub use core::services::{get_current_prices /*, später get_market_chart, ...*/};
pub use core::models::CoinPrice;
