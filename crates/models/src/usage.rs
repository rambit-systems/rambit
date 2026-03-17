use std::ops::Range;

use model::{IndexValue, Model, RecordId};
use serde::{Deserialize, Serialize};
use time::UtcDateTime;

use crate::Org;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Model)]
#[model(
  table = "org_usage",
  index(name = "org_period_start", extract =
    |m| vec![IndexValue::new([
      m.org.to_string(),
      m.period.start.unix_timestamp().to_string()
    ])]
  ),
)]
pub struct OrgUsageEntry {
  /// The entry's ID.
  #[model(id)]
  pub id:  RecordId<OrgUsageEntry>,
  /// The org the usage belongs to.
  pub org: RecordId<Org>,

  /// The measured period.
  pub period: Range<UtcDateTime>,

  /// The amount of egress usage in the period.
  pub egress_byte_count:  u128,
  /// The amount of compute usage in the period.
  pub compute_byte_count: u128,
}
