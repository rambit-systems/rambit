//! Provides the [`AuthDomainService`], the entry point for users,
//! authentication, and authorization logic.

mod cache;
#[cfg(test)]
mod tests;

use std::{sync::Arc, time::Duration};

use axum_login::AuthUser as AxumLoginAuthUser;
pub use axum_login::AuthnBackend;
use domain::{DomainService, authenticate::AuthenticationError};
use miette::IntoDiagnostic;
use models::{
  AuthUser, EmailAddress, User, UserSubmittedAuthCredentials, model::RecordId,
};
use tracing::debug;

use self::cache::ExpiringCache;

/// The authentication session type.
pub type AuthSession = axum_login::AuthSession<AuthDomainService>;

/// A dynamic [`AuthDomainService`] trait object.
#[derive(Clone, Debug)]
pub struct AuthDomainService {
  domain:     DomainService,
  user_cache: Arc<ExpiringCache<RecordId<User>, AuthUser>>,
}

impl AuthDomainService {
  /// Creates a new [`AuthDomainService`].
  #[must_use]
  pub fn new(domain: DomainService) -> Self {
    Self {
      domain,
      user_cache: Arc::new(ExpiringCache::new(Duration::from_secs(0))),
    }
  }
}

impl AuthnBackend for AuthDomainService {
  type Credentials = (EmailAddress, UserSubmittedAuthCredentials);
  type Error = AuthenticationError;
  type User = AuthUser;

  #[tracing::instrument(skip(self))]
  async fn authenticate(
    &self,
    creds: Self::Credentials,
  ) -> Result<Option<Self::User>, Self::Error> {
    self
      .domain
      .user_authenticate(creds.0, creds.1)
      .await
      .map(|u| u.map(Into::into))
  }

  #[tracing::instrument(skip(self))]
  async fn get_user(
    &self,
    id: &<Self::User as AxumLoginAuthUser>::Id,
  ) -> Result<Option<Self::User>, Self::Error> {
    // use cache if it's there
    if let Some(user) = self.user_cache.get(id).await {
      debug!(%id, "user cache hit from AuthDomain");
      return Ok(Some(user));
    }

    // fetch the user from the DB
    debug!(%id, "user cache miss from AuthDomain: fetching user");
    let user = self
      .domain
      .meta()
      .fetch_user_by_id(*id)
      .await
      .into_diagnostic()
      .map_err(AuthenticationError)?
      .map(AuthUser::from);

    if let Some(user) = user {
      // populate cache
      self.user_cache.insert(*id, user.clone()).await;
      Ok(Some(user))
    } else {
      Ok(None)
    }
  }
}
