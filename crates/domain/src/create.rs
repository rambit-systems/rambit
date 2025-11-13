use db::DatabaseError;
use miette::{Context, IntoDiagnostic, Report};
use models::{Cache, EntityName, Org, OrgIdent, RecordId, Store, User};

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
  ) -> Result<Org, Report> {
    let org = Org {
      id:        RecordId::new(),
      org_ident: OrgIdent::Named(org_name),
      owner:     user_id,
    };

    self
      .mutate
      .create_org(&org)
      .await
      .context("failed to create org")?;

    self
      .add_org_to_user(user_id, org.id)
      .await
      .into_diagnostic()
      .context("failed to add user to newly created org")?;

    self
      .mutate
      .switch_active_org(user_id, org.id)
      .await
      .context("failed to switch user active org")?;

    Ok(org)
  }
}
