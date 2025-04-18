//!
//! This module contains functions for generating API documentation for the
//! gRPC services defined in the project.
//!
//! This module uses the `protoc-gen-doc` binary to generate the documentation.
//!
//! For more information, see:
//! <https://github.com/pseudomuto/protoc-gen-doc>
use async_compression::tokio::bufread::GzipDecoder;
use async_tar::Archive;
use futures::TryStreamExt;
use glob::GlobError;
use log::debug;
use log::info;
use std::io;
use std::io::ErrorKind;
use tempfile::tempdir;
use tokio::fs;
use tokio::io::BufReader;
use tokio_util::compat::TokioAsyncReadCompatExt;
use tokio_util::io::StreamReader;

/// This function generates API documentation for the gRPC services defined in
/// the project.  It does this by downloading the protoc-gen-doc binary, and
/// then running it against the protobuf files in the project.  The output is
/// written to the `docs` directory.
///
/// This function is intended to be run as part of a CI/CD pipeline, or as a
/// manual step when updating the API.
pub async fn generate_api_docs() -> anyhow::Result<()> {
  info!("Generating docs...");
  let version = "1.5.1";
  let platform = get_protoc_gen_doc_suffix();
  let url = get_protoc_gen_doc_url(version, &platform);
  let tmp_dir = tempdir()?;
  let tmp_dir_path = tmp_dir.path();

  info!(
    "Downloading protoc-gen-doc version {} for {}",
    version, platform
  );
  let response = reqwest::get(url).await?;
  let reader = response
    .bytes_stream()
    .map_err(|e| io::Error::new(ErrorKind::Other, e));

  let reader = StreamReader::new(reader);
  let reader = BufReader::new(reader);
  let reader = GzipDecoder::new(reader);
  let reader = reader.compat();
  let archive = Archive::new(reader);

  archive.unpack(tmp_dir_path).await?;

  let paths = glob::glob("./src/protocols/**/*.proto")?
    .collect::<Result<Vec<_>, GlobError>>()?;

  let tmp_file_path = format!(
    "{}/protoc-gen-doc",
    tmp_dir_path
      .to_str()
      .ok_or(anyhow::anyhow!("Failed to convert path to string"))?
  );

  let out_dir_path = "./docs";

  {
    let result = fs::remove_dir_all(out_dir_path).await;

    if let Err(e) = result {
      info!("Failed to remove temporary directory: {}", e);
    }
  }

  {
    let result = fs::create_dir_all(out_dir_path).await;

    if let Err(e) = result {
      info!("Failed to create output directory: {}", e);
    }
  }

  info!("Running protoc-gen-doc...");
  let result = std::process::Command::new("protoc")
    .arg(format!("--doc_out={}", out_dir_path))
    .arg("--doc_opt=markdown,index.md")
    .arg(format!("--plugin=protoc-gen-doc={}", tmp_file_path))
    .args(paths)
    .output()?;

  debug!("protoc exited with status: {}", result.status);
  debug!("stdout: {}", String::from_utf8_lossy(&result.stdout));
  debug!("stderr: {}", String::from_utf8_lossy(&result.stderr));

  info!("Generated docs in {}", out_dir_path);
  Ok(())
}

/// Get the URL for the protoc-gen-doc binary, depending on our platform.
fn get_protoc_gen_doc_url(version: &str, suffix: &str) -> String {
  format!("https://github.com/pseudomuto/protoc-gen-doc/releases/download/v{}/protoc-gen-doc_{}_{}.tar.gz", version, version, suffix)
}

/// Get linux x64 suffix for protoc-gen-doc:
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn get_protoc_gen_doc_url() -> String {
  "linux_amd64".to_string()
}

/// Get linux arm64 suffix for protoc-gen-doc:
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
fn get_protoc_gen_doc_suffix() -> String {
  "linux_arm64".to_string()
}

/// Get windows suffix for protoc-gen-doc:
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
fn get_protoc_gen_doc_url() -> String {
  "darwin_amd64".to_string()
}

/// Get macos arm64 suffix for protoc-gen-doc:
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn get_protoc_gen_doc_suffix() -> String {
  "darwin_arm64".to_string()
}
