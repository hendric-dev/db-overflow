use crate::ENV;
use sqlx::{postgres::PgConnection, Connection};

pub async fn establish() -> Result<PgConnection, sqlx::Error> {
  PgConnection::connect(&format!(
    "postgres://{}:{}@{}:{}/{}",
    ENV.db_user, ENV.db_password, ENV.db_host, ENV.db_port, ENV.db_name,
  ))
  .await
}
