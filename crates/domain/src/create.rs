use db::DatabaseError;
use miette::{Context, IntoDiagnostic, Report};
use models::{
  Cache, EmailAddress, EntityName, Org, OrgIdent, RecordId, Store, User,
};

use crate::DomainService;

impl DomainService {
  /// Creates a [`Cache`].
  #[tracing::instrument(skip(self))]
  pub async fn create_cache(
    &self,
    cache: &Cache,
  ) -> Result<RecordId<Cache>, DatabaseError> {
    self.mutate.create_cache(cache).await
  }

  /// Creates a [`Store`].
  #[tracing::instrument(skip(self))]
  pub async fn create_store(
    &self,
    store: &Store,
  ) -> Result<RecordId<Store>, DatabaseError> {
    self.mutate.create_store(store).await
  }

  /// Creates an [`Org`].
  #[tracing::instrument(skip(self))]
  pub async fn create_named_org_with_user(
    &self,
    user_id: RecordId<User>,
    org_name: EntityName,
    billing_email: EmailAddress,
  ) -> Result<Org, Report> {
    let org_id = RecordId::new();

    let customer_id = self
      .billing
      .upsert_customer(org_id, org_name.as_ref(), &billing_email)
      .await
      .context("failed to create customer for org")?;

    let org = Org {
      id: org_id,
      org_ident: OrgIdent::Named(org_name),
      billing_email,
      customer_id,
    };

    self
      .mutate
      .create_org(&org)
      .await
      .context("failed to create org")?;

    self
      .add_org_to_user(user_id, org_id)
      .await
      .into_diagnostic()
      .context("failed to add user to newly created org")?;

    self
      .mutate
      .switch_active_org(user_id, org_id)
      .await
      .context("failed to switch user active org")?;

    Ok(org)
  }
}
