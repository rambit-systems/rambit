//! Entrypoint for domain logic.

mod create;
mod delete_entry;
pub mod download;
pub mod mutate_user;
pub mod narinfo;
mod storage_glue;
pub mod upload;

pub use belt;
pub use billing_domain;
use billing_domain::BillingService;
pub use bytes;
pub use db;
pub use meta_domain;
use meta_domain::MetaService;
pub use models;
use models::PaddleClientSecret;
pub use mutate_domain;
use mutate_domain::MutationService;

/// The domain service type.
#[derive(Debug, Clone)]
pub struct DomainService {
  meta:    MetaService,
  mutate:  MutationService,
  billing: BillingService,
}

impl DomainService {
  /// Create a new [`DomainService`].
  pub fn new(
    meta: MetaService,
    mutate: MutationService,
    billing: BillingService,
  ) -> Self {
    Self {
      meta,
      mutate,
      billing,
    }
  }

  /// Access the internal [`MetaService`].
  pub fn meta(&self) -> &MetaService { &self.meta }

  /// Return the Paddle client secret.
  pub fn paddle_client_secret(&self) -> PaddleClientSecret {
    self.billing.get_client_secret()
  }
}
