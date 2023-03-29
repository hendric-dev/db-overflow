mod cli;
mod database;

use clap::Parser;
use cli::Cli;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  dotenv().ok();
  Cli::parse().run().await?;
  Ok(())
}
