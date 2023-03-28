mod commands;

use clap::Parser;
use commands::Commands;
use once_cell::sync::Lazy;

pub static CLI: Lazy<Cli> = Lazy::new(|| Cli::parse());

#[derive(Parser, Debug)]
#[command(about = "DB Overflow - Insert large amounts of data into a Postgres DB", version)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Option<Commands>,

  #[arg(env = "DB_HOST")]
  #[arg(help = "Database IP address of domain")]
  #[arg(long)]
  pub db_host: String,

  #[arg(env = "DB_NAME")]
  #[arg(help = "Database name")]
  #[arg(long)]
  pub db_name: String,

  #[arg(env = "DB_PORT")]
  #[arg(help = "Database port")]
  #[arg(long)]
  pub db_port: i32,

  #[arg(env = "DB_USER")]
  #[arg(help = "Database username")]
  #[arg(long)]
  pub db_user: String,

  #[arg(env = "DB_PASSWORD")]
  #[arg(help = "Database password")]
  #[arg(long)]
  pub db_password: String,

  #[arg(env = "TABLE")]
  #[arg(help = "Which table to fill")]
  #[arg(long)]
  #[arg(short)]
  pub table: String,
}

impl Cli {
  pub async fn run(&self) -> Result<(), sqlx::Error> {
    match &self.command {
      Some(Commands::Config { output }) => commands::config::execute(output).await,
      Some(Commands::Run { delimiter, quantity }) => commands::run::execute(delimiter, *quantity).await,
      None => Ok(()),
    }
  }
}
