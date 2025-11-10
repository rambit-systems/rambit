use db::DatabaseError;
use miette::{Context, IntoDiagnostic, Report};
use models::{Cache, EmailAddress, EntityName, Org, OrgIdent, RecordId, Store};

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
  pub async fn create_named_org(
    &self,
    id: RecordId<Org>,
    org_name: EntityName,
    billing_email: EmailAddress,
  ) -> Result<Org, Report> {
    let customer_id = self
      .billing
      .create_customer(id, org_name.as_ref(), &billing_email)
      .await
      .context("failed to create customer for org")?;

    let org = Org {
      id,
      org_ident: OrgIdent::Named(org_name),
      billing_email,
      customer_id,
    };

    self.mutate.create_org(&org).await.into_diagnostic()?;

    Ok(org)
  }
}
