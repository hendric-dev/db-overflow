use serde::Deserialize;
use sqlx::postgres::{PgConnection, PgCopyIn};
use std::env;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct TableSchema {
  pub table: String,
  pub columns: Vec<Column>,
}

#[derive(Deserialize, Debug)]
pub struct Column {
  #[serde(rename = "type")]
  pub kind: Option<String>,
  pub value: Option<String>,
}

impl TableSchema {
  pub async fn generate(&self, stream: &mut PgCopyIn<&mut PgConnection>, quantity: i32) -> Result<(), sqlx::Error> {
    let delimiter = env::var("DELIMITER").expect("Failed to get DELIMITER environment variable");

    for _ in 1..=quantity {
      stream.send(self.to_csv(&delimiter).as_bytes()).await?;
      stream.send("\n".as_bytes()).await?;
    }

    Ok(())
  }

  pub fn to_csv(&self, delimiter: &str) -> String {
    self
      .columns
      .iter()
      .map(|column| {
        if column.value.is_some() {
          column.value.clone().unwrap()
        } else {
          self.generate_from_kind(&column.kind.clone().unwrap())
        }
      })
      .collect::<Vec<String>>()
      .join(delimiter)
  }

  fn generate_from_kind(&self, kind: &str) -> String {
    match kind {
      "timestamp" => String::from("now()"),
      "uuid" => Uuid::new_v4().to_string(),
      _ => String::from(""),
    }
  }
}
