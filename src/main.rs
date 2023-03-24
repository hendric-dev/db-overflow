mod database;
mod environment;
mod schema;

use dotenvy::dotenv;
pub use environment::ENV;
use schema::Schema;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  dotenv().ok();

  let schema = Schema::from_file(&ENV.schema_file).expect(&format!("Failed to parse {}", ENV.schema_file));

  let mut connection = database::connect().await?;
  let statement = format!("COPY {} FROM STDIN WITH (DELIMITER '{}', NULL 'NULL')", schema.table, ENV.delimiter);
  let mut stream = connection.copy_in_raw(&statement).await?;

  schema.generate(&mut stream, ENV.quantity).await?;
  stream.finish().await?;

  Ok(())
}
