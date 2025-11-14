use miette::Context;
use models::{Org, PaddleSubscription, PaddleSubscriptionStatus, RecordId};
use tracing::instrument;

use super::helpers::associated_org_id_from_subscription;
use crate::DomainService;

/// A report of an [`Org`]'s [`PaddleSubscription`]s.
pub struct OrgSubscriptionReport {
  /// The org's current subscription, if it exists.
  pub current: Option<PaddleSubscription>,
  /// The list of the org's past subscriptions.
  pub past:    Vec<PaddleSubscription>,
}

impl DomainService {
  /// Generates an [`OrgSubscriptionReport`] for the given org.
  #[instrument(skip(self))]
  pub async fn org_subscription_report(
    &self,
    org_id: RecordId<Org>,
  ) -> miette::Result<OrgSubscriptionReport> {
    let subs = self
      .get_subscriptions_for_org(org_id)
      .await
      .context("failed to get subscriptions for org")?;

    let mut current = Vec::new();
    let mut past = Vec::new();

    for sub in subs {
      match sub.status {
        PaddleSubscriptionStatus::Active
        | PaddleSubscriptionStatus::PastDue
        | PaddleSubscriptionStatus::Paused
        | PaddleSubscriptionStatus::Trialing => {
          current.push(sub);
        }
        PaddleSubscriptionStatus::Canceled => {
          past.push(sub);
        }
        _ => todo!(),
      }
    }

    if current.len() > 1 {
      let err = miette::miette!(
        "multiple current subscriptions found for org {org_id}: {current:?}"
      );
      tracing::error!("{err:?}");
      return Err(err);
    }

    Ok(OrgSubscriptionReport {
      current: current.first().cloned(),
      past,
    })
  }

  /// Gets all Paddle subscriptions for a given [`Org`].
  #[instrument(skip(self))]
  pub async fn get_subscriptions_for_org(
    &self,
    org_id: RecordId<Org>,
  ) -> miette::Result<Vec<PaddleSubscription>> {
    // fetch the org being filtered for
    let org = self
      .meta
      .fetch_org_by_id(org_id)
      .await
      .context("failed to fetch org")?
      .ok_or(miette::miette!("org {org_id} does not exist"))?;

    // pull out the org's owner
    let org_owner_id = org.owner;

    // fetch the org's owner, who holds the customer ID
    let org_owner = self
      .meta
      .fetch_user_by_id(org_owner_id)
      .await
      .context("failed to fetch org owner")?
      .ok_or(miette::miette!(
        "user {org_owner_id} (who is owner of org {org_id}) does not exist"
      ))?;

    // pull out the customer ID
    let customer_id = org_owner.customer_id;

    // find all subscriptions for that customer
    let all_customer_subs = self
      .billing
      .get_all_subscriptions_for_customer(&customer_id)
      .await
      .with_context(|| {
        format!(
          "failed to fetch subscriptions for customer {customer_id} (user \
           {org_owner_id})"
        )
      })?;

    // filter for subscriptions associated to the given org
    let org_associated_subs = all_customer_subs
      .into_iter()
      .filter(|s| {
        associated_org_id_from_subscription(s).is_some_and(|id| id == org_id)
      })
      .collect();

    Ok(org_associated_subs)
  }
}
