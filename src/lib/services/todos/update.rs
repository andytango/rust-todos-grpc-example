//!
//! # Update Todo
//!
//! This module contains the implementation for updating a todo.
use crate::proto;
use crate::services::todos::common::TodoRow;
use crate::update_mask_handler::UpdateMaskHandler;
use anyhow::anyhow;
use sqlx::query_as;
use sqlx::PgPool;

/// Update a todo in the database.
///
/// # Arguments
///
/// * `pool` - The database pool to use.
/// * `request` - The request containing the todo to update.
///
/// # Returns
///
/// A `UpdateTodoResponse` containing the updated todo.
///
pub async fn update_todo(
  pool: PgPool,
  request: proto::v1::todos::UpdateTodoRequest,
) -> anyhow::Result<proto::v1::todos::UpdateTodoResponse> {
  let params = request.todo.ok_or(anyhow!("Todo not provided"))?;

  // We require an update mask to ensure that the update behaviour remains
  // explicit. Otherwise, the default behaviour would be to update all fields,
  // which could lead to unexpected behaviour, particularly if we add new fields
  // in the future.
  let update_mask_paths = request
    .update_mask
    .ok_or(anyhow!("Update mask not provided"))?
    .paths;

  // The update mask handler makes it slightly more convenient to extract the
  // params we want to update.
  let update_mask_handler = UpdateMaskHandler::new(&params, update_mask_paths);

  // Update the todo in the database, returning the result as a TodoRow.
  // Note that we use coalesce to handle optional parameters.  If a parameter is
  // not provided in the update mask, then the existing value will be used.
  // This is important because it means that we don't accidentally overwrite
  // values that the client didn't intend to change. For example, if the client
  // only wants to update the title, then the description and completed fields
  // will remain unchanged.
  let todo = query_as!(
    TodoRow,
    r#"
    update todos
    set title = coalesce($1, todos.title),
        description = coalesce($2, todos.description),
        completed = coalesce($3, todos.completed)
    where todo_id = $4
    returning *
    "#,
    update_mask_handler.get_param("title", |p| &p.title),
    update_mask_handler.get_param("description", |p| &p.description),
    update_mask_handler.get_param("completed", |p| &p.completed),
    params.todo_id
  )
  .fetch_one(&pool)
  .await?;

  // Return the todo wrapped in a protobuf response. The TodoRow is
  // automatically converted to a protobuf Todo by the `into()` method because
  // we have defined the Into trait for this conversion.
  Ok(proto::v1::todos::UpdateTodoResponse {
    todo: Some(todo.into()),
  })
}
