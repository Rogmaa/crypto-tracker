use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "crypto-tracker",
    about = "CLI-Tool zum Abrufen von Kryptowährungspreisen",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Price {
        #[arg(long, value_delimiter = ',')]
        coins: Vec<String>,

        #[arg(long, default_value = "eur")]
        vs_currency: String,
    },

    Watch {
        #[arg(long, value_delimiter = ',')]
        coins: Vec<String>,

        #[arg(long, default_value = "eur")]
        vs_currency: String,

        #[arg(long, default_value_t = 10)]
        interval: u64,
    },
}
