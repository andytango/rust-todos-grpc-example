use crate::proto;
use crate::services::todos::common::TodoRow;
use sqlx::PgPool;
use sqlx::query_as;

/// List all todos. This function takes a database pool and a request object.
/// The request object is not used in this implementation, but it could be used
/// to implement pagination or filtering in the future.
pub async fn list_todos(
  pool: PgPool,
  _request: proto::v1::todos::ListTodosRequest,
) -> anyhow::Result<proto::v1::todos::ListTodosResponse> {
  // Query the database for all todos. We would normally include things like
  // pagination and filtering here, using values from the request object.
  let result = query_as!(
    TodoRow,
    r#"
    select todo_id,
           title,
           description,
           completed,
           created_at,
           updated_at
    from todos
    order by created_at desc
    "#
  )
  .fetch_all(&pool)
  .await?
  .into_iter()
  // Each TodoRecord is automatically converted to a protobuf Todo by the
  // `into()` method because we have defined the Into trait for this conversion.
  .map(|r| r.into())
  // The `collect()` method automatically converts the iterator to the expected
  // vector type based on the downstream usage.
  .collect();

  // Return the todos wrapped in a protobuf response.
  Ok(proto::v1::todos::ListTodosResponse { todos: result })
}
