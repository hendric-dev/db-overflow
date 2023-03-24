use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Column {
  #[serde(rename = "type")]
  pub kind: Option<ColumnType>,
  pub value: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum ColumnType {
  #[serde(alias = "boolean")]
  Boolean,
  #[serde(alias = "character varying")]
  Character,
  #[serde(alias = "date")]
  Date,
  #[serde(alias = "integer")]
  Integer,
  #[serde(alias = "jsonb")]
  Jsonb,
  #[serde(alias = "text")]
  Text,
  #[serde(alias = "time without time zone", alias = "timestamp without time zone")]
  Timestamp,
  #[serde(alias = "uuid")]
  Uuid,
}

impl Column {
  pub fn to_value(&self) -> String {
    self.value.clone().unwrap_or_else(|| self.generate())
  }

  fn generate(&self) -> String {
    match self.kind.as_ref().expect("Called generate on a column with no type") {
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
