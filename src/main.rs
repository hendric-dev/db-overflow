mod cli;
mod database;

use cli::CLI;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  dotenv().ok();
  CLI.run().await?;
  Ok(())
}
