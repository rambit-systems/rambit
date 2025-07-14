use super::generic::{EntityNameFromQueryExtractor, QueryParameter};

pub struct TargetStoreQueryParameter;

impl QueryParameter for TargetStoreQueryParameter {
  const DESCRIPTION: &'static str = "Target store";
  const PARAM_NAME: &'static str = "target_store";
}

pub type TargetStoreExtractor =
  EntityNameFromQueryExtractor<TargetStoreQueryParameter>;
