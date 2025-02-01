use crate::common::sql_datetime_to_proto_timestamp;
use crate::proto;

pub struct TodoRow {
  pub todo_id: String,
  pub title: String,
  pub description: String,
  pub completed: bool,
  pub created_at: sqlx::types::time::OffsetDateTime,
  pub updated_at: sqlx::types::time::OffsetDateTime,
}

impl Into<proto::v1::todos::Todo> for TodoRow {
  fn into(self) -> proto::v1::todos::Todo {
    proto::v1::todos::Todo {
      todo_id: self.todo_id,
      title: self.title,
      description: self.description,
      completed: self.completed,
      created_at: Some(sql_datetime_to_proto_timestamp(self.created_at)),
      updated_at: Some(sql_datetime_to_proto_timestamp(self.updated_at)),
    }
  }
}
