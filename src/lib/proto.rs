pub mod v1 {

  pub mod todos {
    tonic::include_proto!("example.v1.todos");
  }
}

pub const FILE_DESCRIPTOR_SET: &[u8] =
  tonic::include_file_descriptor_set!("service_descriptor");
