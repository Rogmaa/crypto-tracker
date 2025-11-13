use crate::core::models::{CoinPrice, TimeSeriesPoint};
use crate::infra::coingecko;
use std::error::Error;

pub async fn get_current_prices(
    coins: &[String],
    vs_currency: &str,
) -> Result<Vec<CoinPrice>, Box<dyn Error + Send + Sync>> {
    let client = coingecko::make_client();
    coingecko::fetch_simple_prices(&client, coins, vs_currency).await
}

pub async fn get_market_chart(
    coin_id: &str,
    vs_currency: &str,
    days: u32,
) -> Result<Vec<TimeSeriesPoint>, Box<dyn Error + Send + Sync>> {
    let client = coingecko::make_client();
    coingecko::fetch_market_chart(&client, coin_id, vs_currency, days).await
}
