//! # Create Todo
//!
//! This module contains the implementation for creating a new todo.
use crate::proto;
use crate::services::todos::common::TodoRow;
use anyhow::anyhow;
use sqlx::query_as;
use sqlx::PgPool;


/// Create a new todo in the database.
///
/// # Arguments
///
/// * `pool` - The database pool to use.
/// * `request` - The request containing the todo to create.
///
/// # Returns
///
/// A `CreateTodoResponse` containing the created todo.
pub async fn create_todo(
  pool: PgPool,
  request: proto::v1::todos::CreateTodoRequest,
) -> anyhow::Result<proto::v1::todos::CreateTodoResponse> {
  let params = request.todo.ok_or(anyhow!("Todo not provided"))?;

  // Insert the todo into the database, returning the result as a TodoRow.
  let row = query_as!(
    TodoRow,
    r#"
    insert into todos (todo_id, title, description, completed)
    values ($1, $2, $3, $4)
    returning *
    "#,
    params.todo_id,
    params.title,
    params.description,
    params.completed
  )
  .fetch_one(&pool)
  .await?;

  // Return the todo wrapped in a protobuf response. The TodoRow is
  // automatically converted to a protobuf Todo by the `into()` method because
  // we have defined the Into trait for this conversion.
  Ok(proto::v1::todos::CreateTodoResponse {
    todo: Some(row.into()),
  })
}
