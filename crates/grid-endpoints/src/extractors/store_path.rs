use super::generic::{QueryParameter, StorePathFromQueryExtractor};

pub struct StorePathQueryParameter;

impl QueryParameter for StorePathQueryParameter {
  const DESCRIPTION: &'static str = "Store path";
  const PARAM_NAME: &'static str = "store_path";
}

pub type StorePathExtractor =
  StorePathFromQueryExtractor<StorePathQueryParameter>;
