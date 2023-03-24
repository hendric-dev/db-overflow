mod column;

use crate::ENV;
use column::Column;
use serde::Deserialize;
use sqlx::postgres::{PgConnection, PgCopyIn};
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Schema {
  pub table: String,
  pub columns: Vec<Column>,
}

impl Schema {
  pub fn from_file(file: &str) -> Result<Self, serde_json::Error> {
    let data = fs::read_to_string(&file).expect(&format!("Failed to read schema file: {file}"));
    serde_json::from_str::<Self>(&data)
  }

  pub async fn generate(&self, stream: &mut PgCopyIn<&mut PgConnection>, quantity: i32) -> Result<(), sqlx::Error> {
    for _ in 1..=quantity {
      stream.send(self.to_csv(&ENV.delimiter).as_bytes()).await?;
      stream.send("\n".as_bytes()).await?;
    }

    Ok(())
  }

  pub fn to_csv(&self, delimiter: &str) -> String {
    self.columns.iter().map(|column| column.to_value()).collect::<Vec<String>>().join(delimiter)
  }
}
