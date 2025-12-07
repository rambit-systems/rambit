use db::DatabaseError;
use miette::{Context, IntoDiagnostic, Report, miette};
use models::{
  Cache, EmailAddress, EntityName, HumanName, Org, OrgIdent, RecordId, Store,
  User, UserAuthCredentials, UserSubmittedAuthCredentials,
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

/// An error that occurs during user creation.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum CreateUserError {
  /// Indicates that the user's email address is already in use.
  #[error("The email address is already in use: \"{0}\"")]
  EmailAlreadyUsed(EmailAddress),
  /// Indicates that an internal error occurred.
  #[error("Failed to create the user")]
  InternalError(miette::Report),
}

impl DomainService {
  /// Sign up a [`User`].
  #[tracing::instrument(skip(self))]
  pub async fn user_signup(
    &self,
    name: HumanName,
    email: EmailAddress,
    auth: UserSubmittedAuthCredentials,
  ) -> Result<User, CreateUserError> {
    use argon2::PasswordHasher;

    if self
      .meta
      .fetch_user_by_email(email.clone())
      .await
      .into_diagnostic()
      .context("failed to check for conflicting user by email")
      .map_err(CreateUserError::InternalError)?
      .is_some()
    {
      return Err(CreateUserError::EmailAlreadyUsed(email));
    }

    let auth: UserAuthCredentials = match auth {
      UserSubmittedAuthCredentials::Password { password } => {
        let salt = argon2::password_hash::SaltString::generate(
          &mut argon2::password_hash::rand_core::OsRng,
        );
        let argon = argon2::Argon2::default();
        let password_hash = models::PasswordHash(
          argon
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
              CreateUserError::InternalError(miette!(
                "failed to create password hash: {e}"
              ))
            })?
            .to_string(),
        );

        UserAuthCredentials::Password { password_hash }
      }
    };

    let user_id = RecordId::new();

    let customer_id = self
      .billing
      .upsert_customer(user_id, name.as_ref(), &email)
      .await
      .context("failed to create customer for user")
      .map_err(CreateUserError::InternalError)?;

    let org = Org {
      id:        RecordId::new(),
      org_ident: OrgIdent::UserOrg(user_id),
      owner:     user_id,
    };

    let user = User {
      id: user_id,
      personal_org: org.id,
      orgs: Vec::new(),
      name: name.clone(),
      name_abbr: User::abbreviate_name(name),
      email,
      auth,
      active_org_index: 0,
      customer_id,
    };

    self
      .mutate
      .create_org(&org)
      .await
      .into_diagnostic()
      .context("failed to create personal org for user")
      .map_err(CreateUserError::InternalError)?;

    self
      .mutate
      .create_user(&user)
      .await
      .into_diagnostic()
      .context("failed to create user")
      .map_err(CreateUserError::InternalError)?;

    Ok(user)
  }
}
