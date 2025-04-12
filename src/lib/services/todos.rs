//!
//! # Todos Service
//!
//! This module contains the implementation for the todos service.
//!
mod common;
mod create;
mod delete;
mod get;
mod list;
mod update;

use crate::proto::v1::todos::todo_service_server::TodoService;
use crate::proto::v1::todos::todo_service_server::TodoServiceServer;
use crate::proto::v1::todos::*;
use crate::services::todos::create::create_todo;
use crate::services::todos::delete::delete;
use crate::services::todos::get::get_todo;
use crate::services::todos::list::list_todos;
use crate::services::todos::update::update_todo;
use sqlx::PgPool;
use tonic::Request;
use tonic::Response;
use tonic::Status;

pub use common::TodoRow;

/// Service handler struct definition that takes a database pool. Any other
/// required dependencies should be added here.
#[derive(Debug)]
pub struct TodoServiceHandler {
  pool: PgPool,
}

impl TodoServiceHandler {
  /// Create the server instance with this handler so the application level
  /// server code can be kept clean.
  pub fn create_server(pool: PgPool) -> TodoServiceServer<Self> {
    TodoServiceServer::new(Self { pool })
  }
}

/// This is the implementation of our gRPC service.  Each function maps to a
/// method in our protobuf definition.  The implementation details are
/// specific to the service and how it interacts with the database.
#[tonic::async_trait]
impl TodoService for TodoServiceHandler {
  async fn list_todos(
    &self,
    request: Request<ListTodosRequest>,
  ) -> Result<Response<ListTodosResponse>, Status> {
    // Delegate the request handling to a function in a separate module, so that
    // we can keep this file clean.
    let response = list_todos(self.pool.clone(), request.into_inner())
      .await
      .map_err(|e| {
        // Map any errors to a gRPC status message.
        Status::internal(format!("Failed to list todos: \n{}", e.to_string()))
      })?;

    // Wrap the protobuf response in tonic's Response type.
    Ok(Response::new(response))
  }

  async fn get_todo(
    &self,
    request: Request<GetTodoRequest>,
  ) -> Result<Response<GetTodoResponse>, Status> {
    let response = get_todo(self.pool.clone(), request.into_inner())
      .await
      .map_err(|e| {
        Status::internal(format!("Failed to get todo: \n{}", e.to_string()))
      })?;

    Ok(Response::new(response))
  }

  async fn create_todo(
    &self,
    request: Request<CreateTodoRequest>,
  ) -> Result<Response<CreateTodoResponse>, Status> {
    let response = create_todo(self.pool.clone(), request.into_inner())
      .await
      .map_err(|e| {
        Status::internal(format!("Failed to create todo: \n{}", e.to_string()))
      })?;

    Ok(Response::new(response))
  }

  async fn update_todo(
    &self,
    request: Request<UpdateTodoRequest>,
  ) -> Result<Response<UpdateTodoResponse>, Status> {
    let response = update_todo(self.pool.clone(), request.into_inner())
      .await
      .map_err(|e| {
        Status::internal(format!("Failed to update todo: \n{}", e.to_string()))
      })?;

    Ok(Response::new(response))
  }

  async fn delete_todo(
    &self,
    request: Request<DeleteTodoRequest>,
  ) -> Result<Response<DeleteTodoResponse>, Status> {
    let response = delete(self.pool.clone(), request.into_inner())
      .await
      .map_err(|e| {
        Status::internal(format!("Failed to delete todo: \n{}", e.to_string()))
      })?;

    Ok(Response::new(response))
  }
}
