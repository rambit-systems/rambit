mod store_path_from_query;

pub use self::store_path_from_query::*;

pub trait QueryParameter {
  const PARAM_NAME: &'static str;
  const DESCRIPTION: &'static str;
}
