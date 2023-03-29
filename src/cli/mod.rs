mod commands;

use clap::{Args, Parser};
use commands::Commands;

#[derive(Parser, Debug)]
#[command(about = "DB Overflow - Insert large amounts of data into a Postgres DB", version)]
pub struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,

  #[clap(flatten)]
  global_args: GlobalArgs,
}

#[derive(Args, Debug)]
pub struct GlobalArgs {
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

  #[arg(env = "TABLE")]
  #[arg(help = "Which table to fill")]
  #[arg(long)]
  #[arg(short)]
  table: String,
}

impl Cli {
  pub async fn run(&self) -> Result<(), sqlx::Error> {
    match &self.command {
      Some(Commands::Config(options)) => options.execute(&self.global_args).await,
      Some(Commands::Run(options)) => options.execute(&self.global_args).await,
      None => Ok(()),
    }
  }
}
