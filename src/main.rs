mod cli;
mod api;
mod models;

use clap::Parser;
use cli::{Cli, Commands};
use reqwest::Client;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Price { coins, vs_currency } => {
            if coins.is_empty() {
                eprintln!("Fehler: Du musst mindestens einen Coin angeben via --coins btc,eth,sol");
                std::process::exit(1);
            }

            let client = Client::new();

            // API-Call
            match api::fetch_simple_prices(&client, &coins, &vs_currency).await {
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
            println!("Subcommand: watch");
            println!("Coins:       {:?}", coins);
            println!("Vs currency: {}", vs_currency);
            println!("Interval:    {} s", interval);

            // TODO: Später Implementierung des Watch-Modes mit Schleife + Sleep
        }
    }
}
