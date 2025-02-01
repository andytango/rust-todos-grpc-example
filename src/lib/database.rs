use log::info;
use crate::require_environment_variable;
use pg_escape::quote_identifier;
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;
/// Create a new SQLx database pool. This function reads the DATABASE_URL
/// environment variable to determine the connection string to use.  If the
/// environment variable is not set, then the function will return an error.
pub async fn create_database_pool() -> anyhow::Result<PgPool> {
  let options = get_database_pool_base_options()?;
  let pool = PgPool::connect_with(options).await?;
  Ok(pool)
}

/// Create a new database with the given name, using a connection to the
/// postgres database.
pub async fn create_database(database_name: &str) -> anyhow::Result<()> {
  let pool = create_database_pool_for_named_database("postgres").await?;
  let database_name = quote_identifier(database_name);
  let _ = sqlx::query(&format!("create database {database_name}"))
    .execute(&pool)
    .await?;
  pool.close().await;

  Ok(())
}

/// Drop a database with the given name, using a connection to the
/// postgres database.
pub async fn drop_database(database_name: &str) -> anyhow::Result<()> {
  let pool = create_database_pool_for_named_database("postgres").await?;
  let database_name = quote_identifier(database_name);
  let _ = sqlx::query(&format!("drop database {database_name} with (force)"))
    .execute(&pool)
    .await?;
  pool.close().await;
  info!("dropped database");
  Ok(())
}

pub async fn create_database_pool_for_named_database(
  database_name: &str,
) -> anyhow::Result<PgPool> {
  let options = get_database_pool_base_options()?;
  let options = options.database(database_name);
  let pool = PgPool::connect_with(options).await?;

  Ok(pool)
}

/// Get the pool options from the environment variables.
pub fn get_database_pool_base_options() -> anyhow::Result<PgConnectOptions> {
  let options = get_database_options_from_url()?;
  let application_name = require_environment_variable("SERVER_NAME")?;
  let options = options.application_name(&application_name);

  Ok(options)
}

/// Get the database options from the DATABASE_URL environment variable.
fn get_database_options_from_url() -> anyhow::Result<PgConnectOptions> {
  let connection_string = require_environment_variable("DATABASE_URL")?;
  let options: PgConnectOptions = connection_string.parse()?;
  Ok(options)
}
