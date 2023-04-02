mod config;
mod run;

use clap::Subcommand;
use config::Config;
use run::Run;

#[derive(Debug, Subcommand)]
pub enum Commands {
  Config(Config),
  Run(Run),
}
