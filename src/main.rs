mod database;
mod environment;

use database::Table;
use dotenvy::dotenv;
pub use environment::ENV;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  dotenv().ok();

  let mut connection = database::connection::establish().await?;
  let table = Table::discover(&ENV.table, &mut connection).await;

  let statement = format!("COPY {} FROM STDIN WITH (DELIMITER '{}', NULL 'NULL')", table.name, ENV.delimiter);
  let mut stream = connection.copy_in_raw(&statement).await?;

  table.generate(&mut stream, ENV.quantity).await?;
  stream.finish().await?;

  Ok(())
}
