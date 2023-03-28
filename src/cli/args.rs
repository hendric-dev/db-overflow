use clap::Parser;
use once_cell::sync::Lazy;

pub static ARGS: Lazy<Args> = Lazy::new(|| Args::parse());

#[derive(Parser, Debug)]
#[command(about = "DB Overflow - Insert large amounts of data into a Postgres DB", version)]
pub struct Args {
  #[arg(help = "Database IP address of domain", env = "DB_HOST", long)]
  pub db_host: String,

  #[arg(help = "Database name", env = "DB_NAME", long)]
  pub db_name: String,

  #[arg(help = "Database port", env = "DB_PORT", long)]
  pub db_port: i32,

  #[arg(help = "Database username", env = "DB_USER", long)]
  pub db_user: String,

  #[arg(help = "Database password", env = "DB_PASSWORD", long)]
  pub db_password: String,

  #[arg(help = "Delimiter used to separate the data values", env = "DELIMITER", long)]
  pub delimiter: String,

  #[arg(help = "How many records to insert", env = "QUANTITY", short, long)]
  pub quantity: i32,

  #[arg(help = "Which table to fill", env = "TABLE", short, long)]
  pub table: String,
}
