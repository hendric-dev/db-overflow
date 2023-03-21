use sqlx::postgres::{PgConnection, PgCopyIn};
use std::env;
use uuid::Uuid;

struct Event {
  id: Uuid,
  product_id: Uuid,
  storage_notification_id: Uuid,
  transport_id: Uuid,
  timestamp: String,
  data: String,
}

impl Default for Event {
  fn default() -> Self {
    Event {
      id: Uuid::new_v4(),
      product_id: Uuid::new_v4(),
      storage_notification_id: Uuid::new_v4(),
      transport_id: Uuid::new_v4(),
      timestamp: String::from("now()"),
      data: String::from("{}"),
    }
  }
}

impl Event {
  pub fn to_csv(&self, delimiter: &str) -> String {
    format!(
      "{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}",
      self.id, self.product_id, self.storage_notification_id, self.transport_id, self.timestamp, self.data
    )
  }
}

pub async fn generate(stream: &mut PgCopyIn<&mut PgConnection>) -> Result<(), sqlx::Error> {
  let delimiter = env::var("DELIMITER").expect("Failed to get DELIMITER environment variable");

  for _ in 1..=100 {
    stream.send(Event::default().to_csv(&delimiter).as_bytes()).await?;
    stream.send("\n".as_bytes()).await?;
  }

  Ok(())
}
