//! This module provides a handler for update masks.

/// A handler for update masks in API requests.
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

  /// Retrieves a parameter based on the update mask.
  ///
  /// This function checks if the given `param_name` is present in the
  /// `update_mask_paths`. If it is, it uses the provided `param_getter`
  /// function to extract the corresponding parameter from the internal
  /// `params` and returns a clone of it wrapped in `Some`. If the
  /// `param_name` is not found in the `update_mask_paths`, it returns
  /// `None`.
  ///
  /// # Arguments
  ///
  /// * `param_name` - The name of the parameter to retrieve.
  /// * `param_getter` - A function that takes a reference to `T` and
  ///   returns a reference to `U`.
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
