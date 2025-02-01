use database::create_database;
use database::create_database_pool_for_named_database;
use database::get_database_pool_base_options;
use hyper_util::rt::TokioIo;
use sqlx::PgPool;
use std::convert::Infallible;
use std::future::Future;
use std::sync::Arc;
use tempfile::NamedTempFile;
use tempfile::TempPath;
use todos_service::database;
use todos_service::database::drop_database;
use tokio::net::UnixListener;
use tokio::net::UnixStream;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::body::BoxBody;
use tonic::codegen::http::Request;
use tonic::codegen::http::Response;
use tonic::codegen::Service;
use tonic::server::NamedService;
use tonic::transport::Channel;
use tonic::transport::Endpoint;
use tonic::transport::Server;
use tonic::transport::Uri;
use tower::service_fn;
use uuid::Uuid;

///
/// This function is used to create a test database and then drop it when it is
/// no longer needed.  This is useful for testing purposes, as it allows us to
/// create a clean database for each test, without having to worry about
/// cleaning up the database after each test. The database is created with a
/// unique name, so that we don't have to worry about conflicts between tests.
///
/// This function takes a closure as an argument, which is executed with a
/// database pool. The closure should perform the test, and return a result.
/// The result is then used to determine whether the test passed or failed.  If
/// the test fails, then an error is logged, and the database is dropped.  If
/// the test passes, then the database is dropped.  This ensures that we always,
/// drop the database, even if the test fails.
pub fn with_test_database<T, U>(test: T) -> ()
where
  T: (FnOnce(PgPool) -> U) + std::panic::UnwindSafe,
  U: Future<Output = ()>,
{
  // Create the test database and get the database name
  let database_name = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("Failed building the runtime")
    .block_on(async move { create_test_database().await })
    .expect("Failed to create test database");

  let result = std::panic::catch_unwind(|| {
    // Create a new tokio runtime for the test
    tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .expect("Failed building the runtime")
      .block_on(async {
        let pool =
          create_database_pool_for_named_database(&database_name.clone())
            .await
            .expect("Failed to create database pool");

        test(pool).await;
      });
  });

  // Clean up the database
  tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("Failed building the runtime")
    .block_on(async { drop_database(&database_name.clone()).await })
    .expect("Failed to drop test database");

  assert!(result.is_ok());
}

/// Create a new test database, and return the name of the database.
pub async fn create_test_database() -> anyhow::Result<String> {
  dotenvy::dotenv()?;
  let options = get_database_pool_base_options()?;
  let database_name = options
    .get_database()
    .ok_or(anyhow::anyhow!("DATABASE_URL must contain a database name"))?;
  let database_id = Uuid::new_v4().to_string();
  let database_name = format!("{}_{}", database_name, database_id);
  create_database(&database_name).await?;
  let pool = create_database_pool_for_named_database(&database_name).await?;
  sqlx::migrate!().run(&pool).await?;
  pool.close().await;
  Ok(database_name)
}

/// Create a test server and return a channel to connect to it, using a unix
/// domain socket as the transport instead of tcp. This is  useful for testing,
/// as unix sockets can be created faster than tcp sockets and are guaranteed
/// to be unique without having to generate a random port number.
///
/// This function returns a tuple that contains the future that runs the server
/// and the channel that the client can use to connect to the server. The client
/// should be used to make requests to the server and assert its responses,
/// errors and inspect any side effects (e.g. the test database's state).
///
/// At the end of the test, we should await the completion of both the server
/// future and a future wrapping the client test code, allowing the tokio test
/// runtime to run both futures concurrently and then complete successfully only
/// if the client future completes first. For example:
/// ```
/// let (server_future, channel) = create_test_server(
///   // your tonic service here
/// )
/// .await;
///
/// let request_future = async {
///   let mut client = TodoServiceClient::new(channel);
///   // your test case here
/// };
///
/// // Wait for completion, when the client request future completes
/// tokio::select! {
///     _ = server_future => panic!("server returned first"),
///     _ = request_future => (),
/// }
/// ```
///
/// See [this stackoverflow post](https://stackoverflow.com/a/71808401) for
/// original implementation, and [this updated example](https://github.com/hyperium/tonic/blob/master/examples/src/uds/client.rs)
/// in the tonic library for connecting a client to a unix domain socket.
pub async fn create_test_server<T>(
  service: T,
) -> (impl Future<Output = ()>, Channel)
where
  T: Service<Request<BoxBody>, Response = Response<BoxBody>, Error = Infallible>
    + NamedService
    + Clone
    + Send
    + 'static,
  T::Future: Send + 'static,
{
  let socket = NamedTempFile::new().unwrap();
  let path = Arc::new(socket.into_temp_path());
  std::fs::remove_file(&*path).unwrap();

  let serve_future = create_server_future(service, Arc::clone(&path));
  let channel = create_uds_client_channel(Arc::clone(&path)).await;

  (serve_future, channel)
}

/// Create a server future and return it.  This function takes a tonic service
/// as an argument, and returns a future that runs the server. The server is
/// created using a unix domain socket, and the future returned by this function
/// will run until the server is stopped. This allows us to await the
/// termination of the server at the end of each test case.
fn create_server_future<T>(
  service: T,
  path: Arc<TempPath>,
) -> impl Future<Output = ()> + Sized
where
  T: Service<Request<BoxBody>, Response = Response<BoxBody>, Error = Infallible>
    + NamedService
    + Clone
    + Send
    + 'static,
  T::Future: Send + 'static,
{
  let uds = UnixListener::bind(&*path).unwrap();
  let stream = UnixListenerStream::new(uds);

  async {
    let result = Server::builder()
      .add_service(service)
      .serve_with_incoming(stream)
      .await;
    // Server must be running fine
    assert!(result.is_ok());
  }
}

/// Create a client channel to connect to the server using a unix domain socket.
/// This function takes the path to the unix domain socket as an argument, and
/// returns a channel that can be used to connect to the server.
async fn create_uds_client_channel(path: Arc<TempPath>) -> Channel {
  // The URL will be ignored.
  Endpoint::try_from("http://url.any")
    .unwrap()
    .connect_with_connector(service_fn(move |_: Uri| {
      let path = Arc::clone(&path);
      async move {
        Ok::<_, std::io::Error>(TokioIo::new(
          UnixStream::connect(&*path).await?,
        ))
      }
    }))
    .await
    .unwrap()
}
