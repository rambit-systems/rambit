use std::{path::PathBuf, str::FromStr};

use miette::{Context, IntoDiagnostic, Result};
use models::{
  Cache, Org, Store, StoreConfiguration, User,
  dvf::{
    EntityName, HumanName, LocalStorageCredentials, MemoryStorageCredentials,
    RecordId, StrictSlug,
  },
};

use crate::PrimeDomainService;

impl PrimeDomainService {
  /// Add test data to databases.
  pub async fn migrate_test_data(&self, ephemeral_storage: bool) -> Result<()> {
    let org = self
      .org_repo
      .create_model(Org {
        id:   RecordId::from_str("01JXGXSB69BDHNFTSVG2EPW2M3").unwrap(),
        name: EntityName::new(StrictSlug::new("the-federation")),
      })
      .await
      .into_diagnostic()
      .context("failed to create org")?;

    let _user = self
      .user_repo
      .create_model(User {
        id:   RecordId::from_str("01JXGXV4R6VCZWQ2DAYDWR1VXD").unwrap(),
        org:  org.id,
        name: HumanName::try_new("Jean-Luc Picard")
          .expect("failed to create name"),
      })
      .await
      .into_diagnostic()
      .context("failed to create user")?;

    let albert_store = self
      .store_repo
      .create_model(Store {
        id:          RecordId::from_str("01JXGXVF0MVQNGRM565YHM20BC").unwrap(),
        org:         org.id,
        credentials: match ephemeral_storage {
          true => {
            models::dvf::StorageCredentials::Memory(MemoryStorageCredentials)
          }
          false => models::dvf::StorageCredentials::Local(
            LocalStorageCredentials(PathBuf::from("/tmp/rambit-albert-store")),
          ),
        },
        config:      StoreConfiguration {},
        name:        EntityName::new(StrictSlug::new("albert")),
      })
      .await
      .into_diagnostic()
      .context("failed to create store")?;

    let _aaron_cache = self
      .cache_repo
      .create_model(Cache {
        id:            RecordId::from_str("01JXGXVVE6J16590YJT3SP2P6M")
          .unwrap(),
        org:           org.id,
        name:          EntityName::new(StrictSlug::new("aaron")),
        default_store: albert_store.id,
      })
      .await
      .into_diagnostic()
      .context("failed to create cache")?;

    Ok(())
  }
}
