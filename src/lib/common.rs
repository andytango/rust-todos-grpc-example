//! Common functions and types used throughout the application.

use anyhow::anyhow;
use sqlx::types::time::OffsetDateTime;

/// Initialize the common parts of the application, by:
/// - setting up the environment variables
/// - initializing the logger
pub fn init_common() -> anyhow::Result<()> {
  dotenvy::dotenv()?;
  env_logger::init();

  Ok(())
}

/// Require an environment variable to be set.  If the environment variable
/// is not set, then the function will return an error. We use the `anyhow`
/// library to standardise the type of error and also return a more helpful
/// error message than the standard std::env::var function, which omits the
/// variable name.
pub fn require_environment_variable(name: &str) -> anyhow::Result<String> {
  std::env::var(name).map_err(|e| anyhow!("{}: {}", e, name))
}

/// Convert a SQL `OffsetDateTime` to a protobuf `Timestamp`.
/// See <https://buf.build/protocolbuffers/wellknowntypes/file/main:google/protobuf/timestamp.proto#L133>
/// for more information.
pub fn sql_datetime_to_proto_timestamp(
  time: OffsetDateTime,
) -> prost_types::Timestamp {
  prost_types::Timestamp {
    seconds: time.unix_timestamp(),
    nanos: time.nanosecond() as i32,
  }
}
