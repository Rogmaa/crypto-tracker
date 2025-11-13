use crate::core::models::CoinPrice;
use crate::infra::coingecko;
use std::error::Error;

pub async fn get_current_prices(
    coins: &[String],
    vs_currency: &str,
) -> Result<Vec<CoinPrice>, Box<dyn Error + Send + Sync>> {
    let client = coingecko::make_client();
    coingecko::fetch_simple_prices(&client, coins, vs_currency).await
}
