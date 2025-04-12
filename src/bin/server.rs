//! This binary contains the main server for the service.
//!
//! The server can be run in two modes:
//! - TCP: The server will listen on a TCP port.
//! - UDS: The server will listen on a Unix Domain Socket.
//!
//! The mode is determined by the `SERVER_HOST` environment variable. If the
//! variable starts with a `/`, then the server will run in UDS mode. Otherwise,
//! it will run in TCP mode.
//!
//! The server will also create a reflection server, which can be used to
//! introspect the gRPC services.
//!
use anyhow::anyhow;
use log::info;
use server::get_server_address_tcp;
use server::get_server_uds_stream;
use server::set_sigint_handler_uds;
use todos_service::common::init_common;
use todos_service::common::require_environment_variable;
use todos_service::database::create_database_pool;
use todos_service::server;
use todos_service::services::build_server;

///
/// Entrypoint for the server.
///
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_common()?;

  info!("Creating database pool...");
  let database_pool = create_database_pool()
    .await
    .map_err(|e| anyhow!("Failed to create database pool: {}", e))?;

  // Create the reflection server:
  let reflection_server = server::create_reflection_server()
    .map_err(|e| anyhow!("Failed to create reflection server: {}", e))?;

  let host = require_environment_variable("SERVER_HOST")?;

  // If host begins with a slash, host the server on a unix domain socket.
  if host.starts_with("/") {
    let uds_stream = get_server_uds_stream(&host).map_err(|e| {
      anyhow!("Failed to create unix domain socket stream: {}", e)
    })?;

    set_sigint_handler_uds(host.clone());

    info!("Starting server on unix domain socket: {host}...");
    build_server(&database_pool)
      .add_service(reflection_server)
      .serve_with_incoming(uds_stream)
      .await?;
    info!("Server stopped.");
    return Ok(());
  }

  // Otherwise, host the server on a tcp socket.
  let server_address = get_server_address_tcp()
    .map_err(|e| anyhow!("Failed to get server address: {}", e))?;

  info!("Starting server on tcp socket: {server_address}...");
  build_server(&database_pool)
    .add_service(reflection_server)
    .serve(server_address)
    .await?;

  info!("Server stopped.");

  Ok(())
}
