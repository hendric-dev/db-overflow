use super::Columns;
use crate::cli;
use sqlx::postgres::{PgConnection, PgCopyIn};

#[derive(Debug)]
pub struct Table {
  pub name: String,
  pub columns: Columns,
}

impl Table {
  pub async fn discover(name: &str, connection: &mut PgConnection) -> Self {
    Self {
      name: String::from(name),
      columns: Columns::fetch(name, connection)
        .await
        .expect(&format!("Failed to fetch column information for table {name}")),
    }
  }

  pub async fn generate(&self, stream: &mut PgCopyIn<&mut PgConnection>, quantity: i32) -> Result<(), sqlx::Error> {
    for _ in 1..=quantity {
      stream.send(self.to_csv(&cli::ARGS.delimiter).as_bytes()).await?;
      stream.send("\n".as_bytes()).await?;
    }

    Ok(())
  }

  pub fn to_csv(&self, delimiter: &str) -> String {
    self.columns.0.iter().map(|column| column.generate()).collect::<Vec<String>>().join(delimiter)
  }
}
