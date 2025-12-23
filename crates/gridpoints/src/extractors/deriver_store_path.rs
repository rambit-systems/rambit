use super::generic::{QueryParameter, StorePathFromQueryExtractor};

pub struct DeriverStorePathQueryParameter;

impl QueryParameter for DeriverStorePathQueryParameter {
  const DESCRIPTION: &'static str = "Deriver store path";
  const PARAM_NAME: &'static str = "deriver_store_path";
}

pub type DeriverStorePathExtractor =
  StorePathFromQueryExtractor<DeriverStorePathQueryParameter>;
