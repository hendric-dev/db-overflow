mod column;

use column::{Column, ColumnType};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use sqlx::postgres::{PgConnection, PgCopyIn};
use std::env;
use uuid::Uuid;

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
    self
      .columns
      .iter()
      .map(|column| {
        if column.value.is_some() {
          column.value.clone().unwrap()
        } else {
          self.generate_from_kind(column.kind.as_ref().unwrap())
        }
      })
      .collect::<Vec<String>>()
      .join(delimiter)
  }

  fn generate_from_kind(&self, kind: &ColumnType) -> String {
    match kind {
      ColumnType::Boolean => thread_rng().gen_range(0..=1).to_string(),
      ColumnType::Character => thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect::<String>(),
      ColumnType::Date => String::from("now()"),
      ColumnType::Integer => i16::abs(thread_rng().gen::<i16>()).to_string(),
      ColumnType::Jsonb => String::from("{}"),
      ColumnType::Text => thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect::<String>(),
      ColumnType::Timestamp => String::from("now()"),
      ColumnType::Uuid => Uuid::new_v4().to_string(),
    }
  }
}
