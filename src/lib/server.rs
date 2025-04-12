use log::info;
use std::net::SocketAddr;
use tonic_reflection::server::{ServerReflection, ServerReflectionServer};
use tokio_stream::wrappers::UnixListenerStream;
use std::path::Path;
use tokio::net::UnixListener;
use crate::common::require_environment_variable;
use crate::proto;

/// This function creates a gRPC reflection server that allows clients to
/// introspect the gRPC services offered by this server.  This is useful for
/// tools like grpcurl.
///
/// For more information, see:
/// <https://github.com/hyperium/tonic-reflection>
/// <https://grpc.io/docs/guides/reflection>
pub fn create_reflection_server(
) -> anyhow::Result<ServerReflectionServer<impl ServerReflection>> {
  let reflection_service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
    .build_v1()?;

  Ok(reflection_service)
}

/// Set a signal handler to gracefully shutdown the server when a SIGINT
/// signal is received.  This is important for unix domain sockets, as the
/// socket file should be removed when the server is stopped.
pub fn set_sigint_handler_uds(host: String) {
  let _ = ctrlc::set_handler(move || {
    let _ = std::fs::remove_file(&host);
    info!("Server stopped.");
    std::process::exit(0);
  });
}

/// Get the unix domain socket stream that the gRPC server should listen on,
/// using the given path. The function will also remove any existing socket file
/// at the specified path, to ensure that the server starts cleanly.
pub fn get_server_uds_stream(host: &String) -> anyhow::Result<UnixListenerStream> {
  let path = Path::new(&host);
  let _ = std::fs::remove_file(&*path);
  let uds = UnixListener::bind(&*path)?;
  let uds_stream = UnixListenerStream::new(uds);

  Ok(uds_stream)
}

/// Get the address that the gRPC server should listen on. This address is
/// determined by the SERVER_HOST and SERVER_PORT environment variables. If
/// these variables are not set, then the server will fail to start.
pub fn get_server_address_tcp() -> anyhow::Result<SocketAddr> {
  let host = require_environment_variable("SERVER_HOST")?;
  let port = require_environment_variable("SERVER_PORT")?;
  let result = format!("{}:{}", host, port).parse()?;

  Ok(result)
}