use once_cell::sync::Lazy;
use std::env;

pub static ENV: Lazy<Environment> = Lazy::new(|| Environment {
  db_host: get("DB_HOST"),
  db_name: get("DB_NAME"),
  db_port: as_int("DB_PORT"),
  db_user: get("DB_USER"),
  db_password: get("DB_PASSWORD"),
  delimiter: get("DELIMITER"),
  quantity: as_int("QUANTITY"),
  schema_file: get("SCHEMA_FILE"),
});

pub struct Environment {
  pub db_host: String,
  pub db_name: String,
  pub db_port: i32,
  pub db_user: String,
  pub db_password: String,
  pub delimiter: String,
  pub quantity: i32,
  pub schema_file: String,
}

fn as_int(key: &str) -> i32 {
  str::parse::<i32>(&get(key)).expect(&format!("{key} environment variable is not a number"))
}

fn get(key: &str) -> String {
  env::var(key).expect(&format!("Failed to get {key} environment variable"))
}
