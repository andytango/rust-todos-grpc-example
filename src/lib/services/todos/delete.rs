//! # Delete Todo
//!
//! This module contains the implementation for deleting a todo.
use crate::proto;
use sqlx::query;
use sqlx::PgPool;

/// Delete a todo from the database.
///
/// # Arguments
///
/// * `pool` - The database pool to use.
/// * `request` - The request containing the todo to delete.
///
/// # Returns
///
/// A `DeleteTodoResponse` indicating the todo was deleted.
///
pub async fn delete(
  pool: PgPool,
  request: proto::v1::todos::DeleteTodoRequest,
) -> anyhow::Result<proto::v1::todos::DeleteTodoResponse> {
  let todo_id = request.todo_id;

  // As this is a hard delete, we do not need to return the payload according to
  // https://google.aip.dev/135#guidance. However, we do return an error if the
  // record is not found.
  query!(
    r#"
    delete from todos
    where todo_id = $1
    returning 1 as deleted
    "#,
    todo_id
  )
    .fetch_optional(&pool)
    .await?
    .ok_or(anyhow::anyhow!("Todo with id {} not found", todo_id))?;

  // Return the empty proto as a placeholder.
  Ok(proto::v1::todos::DeleteTodoResponse {})
}
