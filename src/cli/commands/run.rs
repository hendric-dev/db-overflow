use crate::{cli::GlobalArgs, database, database::Table};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(about = "Generate data and fill your DB table")]
pub struct Run {
  #[arg(env = "DELIMITER")]
  #[arg(help = "Delimiter used to separate the data values")]
  #[arg(long)]
  delimiter: String,

  #[arg(env = "QUANTITY")]
  #[arg(help = "How many records to insert")]
  #[arg(long)]
  #[arg(short)]
  quantity: i32,
}

impl Run {
  pub async fn execute(&self, global_args: &GlobalArgs) -> Result<(), sqlx::Error> {
    let mut connection = database::connection::establish(
      &global_args.db_name,
      &global_args.db_host,
      &global_args.db_port,
      &global_args.db_user,
      &global_args.db_password,
    )
    .await?;
    let table = Table::discover(&global_args.table, &mut connection).await;

    let statement = format!("COPY {} FROM STDIN WITH (DELIMITER '{}', NULL 'NULL')", table.name, self.delimiter);
    let mut stream = connection.copy_in_raw(&statement).await?;

    table.generate(&mut stream, &self.delimiter, self.quantity).await?;
    stream.finish().await?;

    Ok(())
  }
}
