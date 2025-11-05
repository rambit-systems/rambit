//! Entrypoint for domain logic.

mod create;
mod delete_entry;
pub mod download;
mod migrate;
pub mod mutate_user;
pub mod narinfo;
mod storage_glue;
pub mod upload;

pub use belt;
pub use bytes;
pub use db;
pub use meta_domain;
use meta_domain::MetaService;
pub use models;
pub use mutate_domain;
use mutate_domain::MutationService;

/// The domain service type.
#[derive(Debug, Clone)]
pub struct DomainService {
  meta:   MetaService,
  mutate: MutationService,
}

impl DomainService {
  /// Create a new [`DomainService`].
  pub fn new(meta: MetaService, mutate: MutationService) -> Self {
    Self { meta, mutate }
  }

  /// Create a mocked-up [`DomainService`].
  pub fn new_mock() -> Self {
    Self {
      meta:   MetaService::new_mock(),
      mutate: MutationService::new_mock(),
    }
  }

  /// Access the internal [`MetaService`].
  pub fn meta(&self) -> &MetaService { &self.meta }
}

#[cfg(test)]
mod tests {
  use crate::DomainService;

  impl DomainService {
    pub(crate) async fn mock_domain() -> DomainService {
      let pds = DomainService::new_mock();

      pds
        .migrate_test_data()
        .await
        .expect("failed to migrate test data");

      pds
    }
  }
}
