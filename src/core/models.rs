#[derive(Debug, Clone)]
pub struct CoinPrice {
    pub id: String,
    pub vs_currency: String,
    pub price: f64,
}

#[derive(Debug, Clone)]
pub struct TimeSeriesPoint {
    pub timestamp_ms: i64,
    pub price: f64,
}
