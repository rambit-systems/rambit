use super::generic::{EntityNameFromPathExtractor, PathParameter};

pub struct CacheNamePathParameter;

impl PathParameter for CacheNamePathParameter {
  const DESCRIPTION: &'static str = "Cache name";
  const PARAM_NAME: &'static str = "cache_name";
}

pub type CacheNameExtractor =
  EntityNameFromPathExtractor<CacheNamePathParameter>;
