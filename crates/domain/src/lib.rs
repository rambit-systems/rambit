//! Entrypoint for domain logic.

mod billing;
mod create;
mod delete_entry;
pub mod download;
mod metrics;
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
pub use metrics_domain;
use metrics_domain::MetricsService;
pub use models;
pub use mutate_domain;
use mutate_domain::MutationService;

/// The domain service type.
#[derive(Debug, Clone)]
pub struct DomainService {
  meta:    MetaService,
  mutate:  MutationService,
  billing: BillingService,
  metrics: MetricsService,
}

impl DomainService {
  /// Create a new [`DomainService`].
  pub fn new(
    meta: MetaService,
    mutate: MutationService,
    billing: BillingService,
    metrics: MetricsService,
  ) -> Self {
    Self {
      meta,
      mutate,
      billing,
      metrics,
    }
  }

  /// Access the internal [`MetaService`].
  pub fn meta(&self) -> &MetaService { &self.meta }
}
