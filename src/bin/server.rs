use anyhow::anyhow;
use log::info;
use std::net::SocketAddr;
use todos_service::database::create_database_pool;
use todos_service::init_common;
use todos_service::proto;
use todos_service::require_environment_variable;
use todos_service::services::build_server;
use tonic_reflection::server::ServerReflection;
use tonic_reflection::server::ServerReflectionServer;

/// Server entrypoint
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_common()?;

  info!("Creating database pool...");
  let database_pool = create_database_pool()
    .await
    .map_err(|e| anyhow!("Failed to create database pool: {}", e))?;

  let reflection_server = create_reflection_server()
    .map_err(|e| anyhow!("Failed to create reflection server: {}", e))?;

  let server_address = get_server_address()
    .map_err(|e| anyhow!("Failed to get server address: {}", e))?;

  info!("Starting server...");
  build_server(&database_pool)
    .add_service(reflection_server)
    .serve(server_address)
    .await?;
  info!("Server stopped.");

  Ok(())
}

/// This function creates a gRPC reflection server that allows clients to
/// introspect the gRPC services offered by this server.  This is useful for
/// tools like grpcurl.
///
/// For more information, see:
/// https://github.com/hyperium/tonic-reflection
/// https://grpc.io/docs/guides/reflection
fn create_reflection_server(
) -> anyhow::Result<ServerReflectionServer<impl ServerReflection>> {
  let reflection_service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
    .build_v1()?;

  Ok(reflection_service)
}

/// Get the address that the gRPC server should listen on. This address is
/// determined by the SERVER_HOST and SERVER_PORT environment variables. If
/// these variables are not set, then the server will fail to start.
pub fn get_server_address() -> anyhow::Result<SocketAddr> {
  let host = require_environment_variable("SERVER_HOST")?;
  let port = require_environment_variable("SERVER_PORT")?;
  let result = format!("{}:{}", host, port).parse()?;

  Ok(result)
}
