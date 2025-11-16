use model::{IndexValue, Model, RecordId};
use model_types::{EntityName, PaddleSubscriptionId};
use serde::{Deserialize, Serialize};
use time::UtcDateTime;

use crate::{AuthUser, User};

/// An org.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Model)]
#[model(
  table = "org",
  index(name = "ident", unique, extract = Org::unique_index_ident)
)]
pub struct Org {
  /// The org's ID.
  #[model(id)]
  pub id:        RecordId<Org>,
  /// The org's identifier.
  pub org_ident: OrgIdent,
  /// The org's owner.
  pub owner:     RecordId<User>,
}

impl Org {
  /// Generates the value of the unique [`Org`] index `ident`.
  pub fn unique_index_ident(&self) -> Vec<IndexValue> {
    vec![self.org_ident.index_value()]
  }
}

/// Billing configuration for an [`Org`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrgBilling {
  /// The start of the current billing period
  pub current_billing_period_start: UtcDateTime,
  /// The end of the current billing period
  pub current_billing_period_end:   UtcDateTime,
  /// Describes the state of the org's billing.
  pub billing_state:                OrgBillingState,
}

/// The billing state of an [`Org`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrgBillingState {
  /// The org does not have subscription associated with it yet, and is
  /// subject to quotas of the free tier.
  FreeTier,
  /// The org has a subscription attached to it which will be billed to in
  /// accordance with the org's usage metrics.
  Subscription(PaddleSubscriptionId),
}

/// The public view of [`Org`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PvOrg {
  /// The org's ID.
  pub id:        RecordId<Org>,
  /// The org's identifier.
  pub org_ident: OrgIdent,
  /// The org's owner.
  pub owner:     RecordId<User>,
}

impl PvOrg {
  /// Returns the org's title from the perspective of a user. `None` if user
  /// shouldn't have access.
  pub fn user_facing_title(&self, user: &AuthUser) -> Option<String> {
    match &self.org_ident {
      OrgIdent::Named(entity_name) => Some(entity_name.to_string()),
      OrgIdent::UserOrg(user_id) if *user_id == user.id => {
        Some("Personal Org".to_owned())
      }
      _ => None,
    }
  }
}

impl From<Org> for PvOrg {
  fn from(value: Org) -> Self {
    PvOrg {
      id:        value.id,
      org_ident: value.org_ident,
      owner:     value.owner,
    }
  }
}

/// The public view of the billing configuration for an [`Org`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PvOrgBilling {
  /// The start of the current billing period
  pub current_billing_period_start: UtcDateTime,
  /// The end of the current billing period
  pub current_billing_period_end:   UtcDateTime,
  /// Describes the state of the org's billing.
  pub billing_state:                PvOrgBillingState,
}

impl From<OrgBilling> for PvOrgBilling {
  fn from(value: OrgBilling) -> Self {
    PvOrgBilling {
      current_billing_period_start: value.current_billing_period_start,
      current_billing_period_end:   value.current_billing_period_start,
      billing_state:                value.billing_state.into(),
    }
  }
}

/// The public view of the billing state of an [`Org`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PvOrgBillingState {
  /// The org does not have subscription associated with it yet, and is
  /// subject to quotas of the free tier.
  FreeTier,
  /// The org has a subscription attached to it which will be billed to in
  /// accordance with the org's usage metrics.
  Subscription,
}

impl From<OrgBillingState> for PvOrgBillingState {
  fn from(value: OrgBillingState) -> Self {
    match value {
      OrgBillingState::FreeTier => PvOrgBillingState::FreeTier,
      OrgBillingState::Subscription(_) => PvOrgBillingState::Subscription,
    }
  }
}

/// The [`Org`]'s identifier.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrgIdent {
  /// An [`Org`] identifier using a name.
  Named(EntityName),
  /// An [`Org`] identifier using a user ID.
  UserOrg(RecordId<User>),
}

impl OrgIdent {
  /// Calculates the unique index value for this org ident.
  pub fn index_value(&self) -> IndexValue {
    match self {
      OrgIdent::Named(entity_name) => IndexValue::new_single(entity_name),
      OrgIdent::UserOrg(u) => IndexValue::new_single(format!("user-{}", u)),
    }
  }
}
