use super::Columns;
use crate::progress_bar::ProgressBar;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgConnection, PgCopyIn};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Table {
  pub name: String,
  pub columns: Columns,
}

impl Table {
  pub fn from_file(path: &str) -> Self {
    let file = fs::read_to_string(path).expect(&format!("Failed to load config file {path}"));
    serde_json::from_str(&file).expect(&format!("Failed to parse config {path}"))
  }

  pub async fn discover(name: &str, connection: &mut PgConnection) -> Self {
    Self {
      name: name.to_owned(),
      columns: Columns::fetch(name, connection)
        .await
        .expect(&format!("Failed to fetch column information for table {name}")),
    }
  }

  pub async fn generate(
    &self,
    stream: &mut PgCopyIn<&mut PgConnection>,
    delimiter: &str,
    quantity: i32,
  ) -> Result<(), sqlx::Error> {
    let progress_bar = ProgressBar::new(quantity);
    let chunk_size = usize::try_from(quantity / 100).expect("Unable to convert progress bar step to usize");
    let step = u64::try_from(chunk_size).expect("Failed to convert chunk size to u64");

    for chunk in &(1..=quantity).chunks(chunk_size) {
      for _ in chunk {
        stream.send(format!("{}\n", self.to_csv(delimiter)).as_bytes()).await?;
      }
      progress_bar.increment_by(step);
    }

    progress_bar.finish();
    Ok(())
  }

  pub fn to_csv(&self, delimiter: &str) -> String {
    self.columns.0.iter().map(|column| column.to_csv()).collect::<Vec<String>>().join(delimiter)
  }
}
