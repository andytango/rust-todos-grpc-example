use std::env;
use std::path::PathBuf;

// generated by `sqlx migrate build-script`
fn main() -> Result<(), Box<dyn std::error::Error>> {
  // trigger recompilation when a new migration is added
  println!("cargo:rerun-if-changed=migrations");
  let descriptor_path =
    PathBuf::from(env::var("OUT_DIR").unwrap()).join("service_descriptor.bin");

  tonic_build::configure()
    .build_server(true)
    .file_descriptor_set_path(descriptor_path)
    .compile_protos(&["v1/todos.proto"], &["src/protocols"])?;

  Ok(())
}
