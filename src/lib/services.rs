//! This module exposes a function that builds all the service implementations
//! and adds them to a new tonic server.
use crate::services::todos::TodoServiceHandler;
use sqlx::PgPool;
use tonic::transport::server::Router;
pub use tonic::transport::Server;

pub mod todos;

/// Add all the services to a new tonic server. This will need to be updated 
/// when new services are added.
pub fn build_server(pool: &PgPool) -> Router {
  let mut server = Server::builder();

  server.add_service(TodoServiceHandler::create_server(pool.clone()))
}
