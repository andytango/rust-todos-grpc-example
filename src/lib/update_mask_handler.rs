pub struct UpdateMaskHandler<T: Clone> {
  params: T,
  update_mask_paths: Vec<String>,
}

impl<T: Clone> UpdateMaskHandler<T> {
  pub fn new(params: &T, update_mask_paths: Vec<String>) -> Self {
    Self {
      params: params.clone(),
      update_mask_paths,
    }
  }

  pub fn get_param<U: Clone>(
    &self,
    param_name: &str,
    param_getter: fn(&T) -> &U,
  ) -> Option<U> {
    if self.update_mask_paths.contains(&param_name.to_string()) {
      return Some(param_getter(&self.params).clone());
    }

    None
  }
}