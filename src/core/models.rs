#[derive(Debug, Clone)]
pub struct CoinPrice {
    pub id: String,
    pub vs_currency: String,
    pub price: f64,
}