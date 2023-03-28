mod cli;
mod database;

use database::Table;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  dotenv().ok();

  let mut connection = database::connection::establish().await?;
  let table = Table::discover(&cli::ARGS.table, &mut connection).await;

  let statement = format!("COPY {} FROM STDIN WITH (DELIMITER '{}', NULL 'NULL')", table.name, cli::ARGS.delimiter);
  let mut stream = connection.copy_in_raw(&statement).await?;

  table.generate(&mut stream, cli::ARGS.quantity).await?;
  stream.finish().await?;

  Ok(())
}
