use sqlx::{postgres::PgConnection, Connection};
use std::env;

pub async fn connect() -> Result<PgConnection, sqlx::Error> {
  PgConnection::connect(&format!(
    "postgres://{}:{}@{}:{}/{}",
    env::var("DB_USER").expect("Failed to get DB_USER environment variable"),
    env::var("DB_PASS").expect("Failed to get DB_PASS environment variable"),
    env::var("DB_HOST").expect("Failed to get DB_HOST environment variable"),
    env::var("DB_PORT").expect("Failed to get DB_PORT environment variable"),
    env::var("DB_NAME").expect("Failed to get DB_NAME environment variable"),
  ))
  .await
}
