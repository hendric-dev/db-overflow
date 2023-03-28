pub mod config;
pub mod run;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
  #[command(about = "Creates a config file from a DB table for more fine grained customizations")]
  Config {
    #[arg(env = "OUTPUT")]
    #[arg(help = "Path where the config file is created")]
    #[arg(long)]
    #[arg(short)]
    output: String,
  },
  #[command(about = "Generate data and fill your DB table")]
  Run {
    #[arg(env = "DELIMITER")]
    #[arg(help = "Delimiter used to separate the data values")]
    #[arg(long)]
    delimiter: String,

    #[arg(env = "QUANTITY")]
    #[arg(help = "How many records to insert")]
    #[arg(long)]
    #[arg(short)]
    quantity: i32,
  },
}
