mod cli;
mod database;
mod progress_bar;

use clap::Parser;
use cli::Cli;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  dotenv().ok();
  Cli::parse().run().await?;
  Ok(())
}
