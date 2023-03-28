use crate::CLI;
use sqlx::{postgres::PgConnection, Connection};

pub async fn establish() -> Result<PgConnection, sqlx::Error> {
  PgConnection::connect(&format!(
    "postgres://{}:{}@{}:{}/{}",
    CLI.db_user, CLI.db_password, CLI.db_host, CLI.db_port, CLI.db_name,
  ))
  .await
}
