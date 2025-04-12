//! Common types and functions for the todos service.

use crate::common::sql_datetime_to_proto_timestamp;
use crate::proto;

/// Represents a row in the `todos` table.
///
/// # Fields
///
/// * `todo_id` - The unique identifier of the todo.
/// * `title` - The title of the todo.
/// * `description` - The description of the todo.
/// * `completed` - Whether the todo is completed.
/// * `created_at` - The timestamp when the todo was created.
/// * `updated_at` - The timestamp when the todo was last updated.
pub struct TodoRow {
  pub todo_id: String,
  pub title: String,
  pub description: String,
  pub completed: bool,
  pub created_at: sqlx::types::time::OffsetDateTime,
  pub updated_at: sqlx::types::time::OffsetDateTime,
}

/// Converts a `TodoRow` to a `proto::v1::todos::Todo`.
///
/// # Arguments
///
/// * `self` - The `TodoRow` to convert.
///
/// # Returns
///
/// A `proto::v1::todos::Todo` representing the `TodoRow`.
///
/// # Examples
///
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
