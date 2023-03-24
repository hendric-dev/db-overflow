mod column;

use column::Column;
use serde::Deserialize;
use sqlx::postgres::{PgConnection, PgCopyIn};
use std::env;

#[derive(Deserialize, Debug)]
pub struct Schema {
  pub table: String,
  pub columns: Vec<Column>,
}

impl Schema {
  pub async fn generate(&self, stream: &mut PgCopyIn<&mut PgConnection>, quantity: i32) -> Result<(), sqlx::Error> {
    let delimiter = env::var("DELIMITER").expect("Failed to get DELIMITER environment variable");

    for _ in 1..=quantity {
      stream.send(self.to_csv(&delimiter).as_bytes()).await?;
      stream.send("\n".as_bytes()).await?;
    }

    Ok(())
  }

  pub fn to_csv(&self, delimiter: &str) -> String {
    self.columns.iter().map(|column| column.to_value()).collect::<Vec<String>>().join(delimiter)
  }
}
