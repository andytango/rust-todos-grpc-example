//! This binary contains the command line interface for the service. The CLI
//! should be used for development, maintenance and administrative tasks, as
//! well as any automated CI processes that cannot directly be supported by
//! cargo or the rust toolchain.
//!
//! Where we use third party binaries, these should ideally be wrapped in this
//! CLI, downloading the appropriate version to this repository's `/bin` 
//! directory.
//!
//! For example, the `protoc-gen-doc` binary is used to generate API
//! documentation, and this is wrapped in the `generate-api-docs` command.
//!
//! This ensures that developers have a consistent experience and minimises
//! effort when setting up a new development environment.
//!

use clap::Parser;
use clap::ValueEnum;
use std::future::Future;
use todos_service::api_docs;
use todos_service::common::init_common;

/// CLI Arguments, for the clap argument parser. See:
/// <https://github.com/clap-rs/clap> for more information.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  command: Command,
}

/// Enum for the different commands the CLI can run.
#[derive(Debug, ValueEnum, Clone)]
enum Command {
  GenerateApiDocs,
}

/// Entrypoint for the CLI. We start a tokio runtime so that the CLI can
/// run any async code if needed.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  init_common()?;
  run_command(Args::parse()).await?;
  Ok(())
}

fn run_command(args: Args) -> impl Future<Output = anyhow::Result<()>> {
  match args.command {
    Command::GenerateApiDocs => api_docs::generate_api_docs(),
  }
}
