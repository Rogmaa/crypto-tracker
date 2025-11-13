pub mod cli;
pub mod api;
pub mod models;

use reqwest::Client;
use crate::models::CoinPrice;

pub async fn get_current_prices(
    coins: &[String],
    vs_currency: &str,
) -> Result<Vec<CoinPrice>, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    crate::api::fetch_simple_prices(&client, coins, vs_currency).await
}
