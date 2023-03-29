mod commands;

use clap::Parser;
use commands::Commands;

#[derive(Parser, Debug)]
#[command(about = "DB Overflow - Insert large amounts of data into a Postgres DB", version)]
pub struct Cli {
  #[command(subcommand)]
  command: Commands,
}

impl Cli {
  pub async fn run(&self) -> Result<(), sqlx::Error> {
    match &self.command {
      Commands::Config(options) => options.execute().await,
      Commands::Run(options) => options.execute().await,
    }
  }
}
