use crate::{database, database::Table};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(about = "Generate data and fill your DB table")]
pub struct Run {
  #[arg(env = "SCHEMA_FILE")]
  #[arg(help = "Optional path to a table schema file to customize data generation")]
  #[arg(long)]
  #[arg(short)]
  config: Option<String>,

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

  #[arg(env = "DELIMITER")]
  #[arg(help = "Delimiter used to separate the data values")]
  #[arg(long)]
  delimiter: String,

  #[arg(env = "QUANTITY")]
  #[arg(help = "How many records to insert")]
  #[arg(long)]
  #[arg(short)]
  quantity: i32,

  #[arg(env = "TABLE")]
  #[arg(help = "Which table to fill")]
  #[arg(long)]
  #[arg(short)]
  table: String,
}

impl Run {
  pub async fn execute(&self) -> Result<(), sqlx::Error> {
    let mut connection =
      database::connection::establish(&self.db_name, &self.db_host, &self.db_port, &self.db_user, &self.db_password)
        .await?;

    let table: Table = match &self.config {
      Some(path) => Table::from_file(&path),
      None => Table::discover(&self.table, &mut connection).await,
    };

    let statement = format!("COPY {} FROM STDIN WITH (DELIMITER '{}', NULL 'NULL')", table.name, self.delimiter);
    let mut stream = connection.copy_in_raw(&statement).await?;

    table.generate(&mut stream, &self.delimiter, self.quantity).await?;
    stream.finish().await?;

    Ok(())
  }
}
