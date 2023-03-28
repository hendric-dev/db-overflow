use crate::cli;
use sqlx::{postgres::PgConnection, Connection};

pub async fn establish() -> Result<PgConnection, sqlx::Error> {
  PgConnection::connect(&format!(
    "postgres://{}:{}@{}:{}/{}",
    cli::ARGS.db_user,
    cli::ARGS.db_password,
    cli::ARGS.db_host,
    cli::ARGS.db_port,
    cli::ARGS.db_name,
  ))
  .await
}
