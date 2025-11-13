use crate::core::models::CoinPrice;
use reqwest::Client;

const COINGECKO_BASE_URL: &str = "https://api.coingecko.com/api/v3";

pub fn make_client() -> Client {
    Client::new()
}

pub async fn fetch_simple_prices(
    client: &Client,
    coins: &[String],
    vs_currency: &str,
) -> Result<Vec<CoinPrice>, Box<dyn std::error::Error + Send + Sync>> {
    if coins.is_empty() {
        return Ok(Vec::new());
    }

    let ids_param = coins.join(",");
    let vs_param = vs_currency;

    let url = format!("{}/simple/price", COINGECKO_BASE_URL);

    let resp = client
        .get(&url)
        .query(&[
            ("ids", ids_param.as_str()),
            ("vs_currencies", vs_param),
            ("include_market_cap", "false"),
            ("include_24hr_vol", "false"),
            ("include_24hr_change", "false"),
            ("include_last_updated_at", "false"),
        ])
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(format!("Coingecko HTTP-Fehler: {}", resp.status()).into());
    }

    let json_value: serde_json::Value = resp.json().await?;
    let obj = json_value
        .as_object()
        .ok_or("Unerwartetes JSON-Format von Coingecko (kein Objekt)")?;

    let mut result = Vec::new();

    for coin_id in coins {
        if let Some(coin_entry) = obj.get(coin_id) {
            if let Some(price_obj) = coin_entry.as_object() {
                if let Some(price_value) = price_obj.get(vs_param) {
                    if let Some(price_f64) = price_value.as_f64() {
                        result.push(CoinPrice {
                            id: coin_id.clone(),
                            vs_currency: vs_param.to_string(),
                            price: price_f64,
                        });
                    }
                }
            }
        }
    }

    Ok(result)
}
