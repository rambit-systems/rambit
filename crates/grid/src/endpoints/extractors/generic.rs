mod entity_name_from_query;
mod store_path_from_query;

pub use self::{entity_name_from_query::*, store_path_from_query::*};

pub trait QueryParameter {
  const PARAM_NAME: &'static str;
  const DESCRIPTION: &'static str;
}
