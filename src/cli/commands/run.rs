use crate::{database, database::Table, CLI};

pub async fn execute(delimiter: &str, quantity: i32) -> Result<(), sqlx::Error> {
  let mut connection = database::connection::establish().await?;
  let table = Table::discover(&CLI.table, &mut connection).await;

  let statement = format!("COPY {} FROM STDIN WITH (DELIMITER '{}', NULL 'NULL')", table.name, delimiter);
  let mut stream = connection.copy_in_raw(&statement).await?;

  table.generate(&mut stream, delimiter, quantity).await?;
  stream.finish().await?;

  Ok(())
}
