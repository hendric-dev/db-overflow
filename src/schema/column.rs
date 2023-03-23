use serde::Deserialize;

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
