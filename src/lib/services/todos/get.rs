use crate::proto;
use crate::services::todos::common::TodoRow;
use sqlx::query_as;
use sqlx::PgPool;

/// Get a todo by its ID. This function takes a database pool and a request
/// object. The request object contains the ID of the todo to retrieve.
/// If the todo is not found, then the function will return an error.
pub async fn get_todo(
  pool: PgPool,
  request: proto::v1::todos::GetTodoRequest,
) -> anyhow::Result<proto::v1::todos::GetTodoResponse> {
  let row = query_as!(
    TodoRow,
    r#"
    select todo_id,
           title,
           description,
           completed,
           created_at,
           updated_at
    from todos
    where todo_id = $1
    "#,
    request.todo_id
  )
  .fetch_optional(&pool)
  .await?;

  // If the row is not found, then return an error.
  let row = row.ok_or(anyhow::anyhow!(
    "Todo with id {} not found",
    request.todo_id
  ))?;

  // Return the todo wrapped in a protobuf response. The TodoRecord is 
  // automatically converted to a protobuf Todo, because we have defined the
  // Into trait for this conversion.
  Ok(proto::v1::todos::GetTodoResponse {
    todo: Some(row.into()),
  })
}
