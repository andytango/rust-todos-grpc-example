mod common;

use crate::common::create_test_server;
use common::with_test_database;
use sqlx::query;
use sqlx::query_as;
use sqlx::PgPool;
use todos_service::proto;
use todos_service::proto::v1::todos::todo_service_client::TodoServiceClient;
use todos_service::proto::v1::todos::ListTodosRequest;
use todos_service::services::todos::TodoRow;
use todos_service::services::todos::TodoServiceHandler;

#[test]
pub fn list_todos() {
  with_test_database(|pool| async move {
    let (server_future, channel) =
      create_test_server(TodoServiceHandler::create_server(pool.clone())).await;

    let request_future = async {
      let mut client = TodoServiceClient::new(channel);

      let response_before =
        client.list_todos(ListTodosRequest {}).await.unwrap();
      assert_eq!(response_before.into_inner().todos.len(), 0);

      create_test_record(&pool).await;

      let response_after =
        client.list_todos(ListTodosRequest {}).await.unwrap();
      assert_eq!(response_after.into_inner().todos.len(), 1);
    };

    // Wait for completion, when the client request future completes
    tokio::select! {
        _ = server_future => panic!("server returned first"),
        _ = request_future => (),
    }
  });
}

#[test]
pub fn create_todo() {
  with_test_database(|pool| async move {
    let (server_future, channel) =
      create_test_server(TodoServiceHandler::create_server(pool.clone())).await;

    let request_future = async {
      let mut client = TodoServiceClient::new(channel);

      let exists_before = does_test_record_exist(&pool).await;
      assert!(!exists_before);

      let response = client
        .create_todo(proto::v1::todos::CreateTodoRequest {
          todo: Some(proto::v1::todos::Todo {
            todo_id: "test-id".to_string(),
            title: "test-title".to_string(),
            description: "test-description".to_string(),
            completed: false,
            created_at: None,
            updated_at: None,
          }),
        })
        .await;

      assert!(response.is_ok());

      let exists_after = does_test_record_exist(&pool).await;
      assert!(exists_after);
    };

    // Wait for completion, when the client request future completes
    tokio::select! {
        _ = server_future => panic!("server returned first"),
        _ = request_future => (),
    }
  });
}

#[test]
pub fn update_todo() {
  with_test_database(|pool| async move {
    let (server_future, channel) =
      create_test_server(TodoServiceHandler::create_server(pool.clone())).await;

    let request_future = async {
      let mut client = TodoServiceClient::new(channel);

      create_test_record(&pool).await;

      let response = client
        .update_todo(proto::v1::todos::UpdateTodoRequest {
          todo: Some(proto::v1::todos::Todo {
            todo_id: "test-id".to_string(),
            title: "updated-title".to_string(),
            description: String::new(),
            completed: false,
            created_at: None,
            updated_at: None,
          }),
          update_mask: Some(prost_types::FieldMask {
            paths: vec!["title".to_string()],
            ..Default::default()
          }),
        })
        .await;

      assert!(response.is_ok());

      let exists_after = does_test_record_exist(&pool).await;
      assert!(exists_after);

      let record = select_test_record(&pool).await.unwrap();
      assert_eq!(record.title, "updated-title");
    };

    // Wait for completion, when the client request future completes
    tokio::select! {
        _ = server_future => panic!("server returned first"),
        _ = request_future => (),
    }
  })
}

#[test]
pub fn delete_todo() {
  with_test_database(|pool| async move {
    let (server_future, channel) =
      create_test_server(TodoServiceHandler::create_server(pool.clone())).await;

    let request_future = async {
      let mut client = TodoServiceClient::new(channel);

      create_test_record(&pool).await;

      let exists_before = does_test_record_exist(&pool).await;
      assert!(exists_before);

      let response = client
        .delete_todo(proto::v1::todos::DeleteTodoRequest {
          todo_id: "test-id".to_string(),
        })
        .await;

      assert!(response.is_ok());

      let exists_after = does_test_record_exist(&pool).await;
      assert!(!exists_after);
    };

    // Wait for completion, when the client request future completes
    tokio::select! {
        _ = server_future => panic!("server returned first"),
        _ = request_future => (),
    }
  })
}

async fn does_test_record_exist(pool: &PgPool) -> bool {
  query!(
    r#"
    select exists(
      select 1 from todos
      where todo_id = 'test-id'
    ) as "record_exists!"
    "#
  )
  .fetch_one(pool)
  .await
  .unwrap()
  .record_exists
}

async fn create_test_record(pool: &PgPool) {
  query!(
    r#"
    insert into todos (todo_id, title, description, completed)
    values ('test-id', 'test-title', 'test-description', false)
    "#
  )
  .execute(pool)
  .await
  .unwrap();
}

async fn select_test_record(pool: &PgPool) -> Option<TodoRow> {
  query_as!(
    TodoRow,
    r#"
    select todo_id,
           title,
           description,
           completed,
           created_at,
           updated_at
    from todos
    where todo_id = 'test-id'
    "#
  )
  .fetch_optional(pool)
  .await
  .unwrap()
}
