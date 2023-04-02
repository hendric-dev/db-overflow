use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{
  postgres::{PgConnection, PgRow},
  query, Row,
};
use std::str::FromStr;
use strum_macros::EnumString;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Columns(pub Vec<Column>);

impl Columns {
  pub async fn fetch(table_name: &str, connection: &mut PgConnection) -> Result<Self, sqlx::Error> {
    let columns = query(
      "SELECT column_name as name, udt_name as data_type FROM information_schema.columns WHERE table_name = $1 ORDER BY ordinal_position",
    )
    .bind(table_name)
    .map(|row: PgRow| {
      Column {
        name: row.get("name"),
        data_type: DataType::from_str(&row.get::<String, &str>("data_type")).expect("Failed to convert column type to DataType"),
        value: None,
      }
    })
    .fetch_all(connection)
    .await?;

    Ok(Columns(columns))
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Column {
  pub name: String,
  pub data_type: DataType,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<String>,
}

#[derive(Debug, Deserialize, EnumString, Serialize)]
#[strum(ascii_case_insensitive)]
pub enum DataType {
  #[strum(serialize = "bool")]
  Boolean,
  #[strum(serialize = "varchar")]
  Character,
  Date,
  #[strum(serialize = "int4")]
  Integer,
  Jsonb,
  Text,
  Timestamp,
  Uuid,
}

impl Column {
  pub fn to_csv(&self) -> String {
    match self.value.as_ref() {
      Some(value) => value.to_owned(),
      None => self.generate(),
    }
  }

  fn generate(&self) -> String {
    match self.data_type {
      DataType::Boolean => thread_rng().gen_range(0..=1).to_string(),
      DataType::Character => thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect(),
      DataType::Date => "now()".to_owned(),
      DataType::Integer => thread_rng().gen_range(0..i16::MAX).to_string(),
      DataType::Jsonb => "{}".to_owned(),
      DataType::Text => thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect(),
      DataType::Timestamp => "now()".to_owned(),
      DataType::Uuid => Uuid::new_v4().to_string(),
    }
  }
}
