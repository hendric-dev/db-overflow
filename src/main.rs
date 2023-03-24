mod database;
mod schema;

use dotenvy::dotenv;
use schema::Schema;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
  dotenv().ok();

  let schema_file = env::var("SCHEMA_FILE").expect("Failed to get SCHEMA_FILE environment variable");
  let schema = Schema::from_file(&schema_file).expect(&format!("Failed to parse {schema_file}"));

  let delimiter = env::var("DELIMITER").expect("Failed to get DELIMITER environment variable");
  let quantity = str::parse::<i32>(&env::var("QUANTITY").expect("Failed to get QUANTITY environment variable"))
    .expect("QUANTITY environment variable is not a number");

  let mut connection = database::connect().await?;
  let statement = format!("COPY {} FROM STDIN WITH (DELIMITER '{delimiter}', NULL 'NULL')", schema.table);
  let mut stream = connection.copy_in_raw(&statement).await?;

  schema.generate(&mut stream, quantity).await?;
  stream.finish().await?;

  Ok(())
}
