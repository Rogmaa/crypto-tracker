use clap::Parser;

use crypto_tracker::cli::{Cli, Commands};
use crypto_tracker::get_current_prices;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Price { coins, vs_currency } => {
            if coins.is_empty() {
                eprintln!("Fehler: Du musst mindestens einen Coin angeben via --coins btc,eth,sol");
                std::process::exit(1);
            }

            match get_current_prices(&coins, &vs_currency).await {
                Ok(prices) => {
                    if prices.is_empty() {
                        println!("Keine Preise erhalten. Prüfe Coin-IDs und Währung.");
                        return;
                    }

                    println!("Aktuelle Preise (Quelle: Coingecko):");
                    for p in prices {
                        println!(
                            "- {:<15} {:>12.2} {}",
                            p.id,
                            p.price,
                            p.vs_currency.to_uppercase()
                        );
                    }
                }
                Err(err) => {
                    eprintln!("Fehler beim Abrufen der Preise: {}", err);
                    std::process::exit(1);
                }
            }
        }

        Commands::Watch {
            coins,
            vs_currency,
            interval,
        } => {
            println!("Watch-Mode noch nicht implementiert.");
            println!("Coins: {:?}, vs_currency: {}, interval: {}s", coins, vs_currency, interval);
        }
    }
}
