mod database;
mod producers;

use dotenvy::dotenv;
use producers::events;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  dotenv().ok();

  let delimiter = env::var("DELIMITER").expect("Failed to get DELIMITER environment variable");
  let table = env::var("TABLE").expect("Failed to get TABLE environment variable");

  let mut connection = database::connect().await?;
  let statement = format!("COPY {table} FROM STDIN WITH (DELIMITER '{delimiter}', NULL 'NULL')");
  let mut stream = connection.copy_in_raw(&statement).await?;
  events::generate(&mut stream).await?;
  stream.finish().await?;

  Ok(())
}
