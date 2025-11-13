use clap::Parser;

use crypto_tracker::core::services::get_current_prices;
use crypto_tracker::presentation::cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Price { coins, vs_currency } => {
            if coins.is_empty() {
                eprintln!("Fehler: Du musst mindestens einen Coin angeben via --coins bitcoin,ethereum");
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
        Commands::Watch { .. } => {
            println!("Watch-Mode noch nicht implementiert.");
        }
    }
}
