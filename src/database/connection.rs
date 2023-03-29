use sqlx::{postgres::PgConnection, Connection};

pub async fn establish(
  db_name: &str,
  db_host: &str,
  db_port: &i32,
  db_user: &str,
  db_password: &str,
) -> Result<PgConnection, sqlx::Error> {
  PgConnection::connect(&format!("postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}",)).await
}
