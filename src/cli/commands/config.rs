use crate::cli::GlobalArgs;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(about = "Creates a config file from a DB table for more fine grained customizations")]
pub struct Config {
  #[arg(env = "OUTPUT")]
  #[arg(help = "Path where the config file is created")]
  #[arg(long)]
  #[arg(short)]
  output: String,
}

impl Config {
  pub async fn execute(&self, global_args: &GlobalArgs) -> Result<(), sqlx::Error> {
    println!("{:#?}", global_args);
    Ok(())
  }
}
