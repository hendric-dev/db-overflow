use crate::{database, database::Table};
use clap::Parser;
use std::{fs, io};

#[derive(Debug, Parser)]
#[command(about = "Creates a config file from a DB table for more fine grained customizations")]
pub struct Config {
  #[arg(env = "DB_HOST")]
  #[arg(help = "Database IP address of domain")]
  #[arg(long)]
  db_host: String,

  #[arg(env = "DB_NAME")]
  #[arg(help = "Database name")]
  #[arg(long)]
  db_name: String,

  #[arg(env = "DB_PORT")]
  #[arg(help = "Database port")]
  #[arg(long)]
  db_port: i32,

  #[arg(env = "DB_USER")]
  #[arg(help = "Database username")]
  #[arg(long)]
  db_user: String,

  #[arg(env = "DB_PASSWORD")]
  #[arg(help = "Database password")]
  #[arg(long)]
  db_password: String,

  #[arg(env = "OUTPUT")]
  #[arg(help = "Path where the config file is created")]
  #[arg(long)]
  #[arg(short)]
  output: String,

  #[arg(env = "TABLE")]
  #[arg(help = "Which table to fill")]
  #[arg(long)]
  #[arg(short)]
  table: String,
}

impl Config {
  pub async fn execute(&self) -> Result<(), sqlx::Error> {
    let mut connection =
      database::connection::establish(&self.db_name, &self.db_host, &self.db_port, &self.db_user, &self.db_password)
        .await?;
    let table = Table::discover(&self.table, &mut connection).await;

    self.write(&table).expect(&format!("Failed to write table schema for {}", table.name));

    Ok(())
  }

  fn write(&self, table: &Table) -> Result<(), io::Error> {
    fs::write(
      &self.output,
      serde_json::to_string_pretty(table).expect(&format!("Failed to serialize the table: {}", table.name)),
    )
  }
}
