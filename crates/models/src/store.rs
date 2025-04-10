use serde::{Deserialize, Serialize};

use crate::{Model, OrgRecordId, RecordId, StorageCredentials};

/// The [`Store`] table name.
pub const STORE_TABLE_NAME: &str = "store";

/// A store record ID.
pub type StoreRecordId = RecordId<Store>;

/// A store.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Store {
  /// The store's ID.
  pub id:                 StoreRecordId,
  /// The store's nickname.
  pub nickname:           dvf::EntityNickname,
  /// The store's credentials.
  pub credentials:        StorageCredentials,
  /// The store's compression configuration.
  pub compression_config: dvf::CompressionConfig,
  /// The [`Org`](crate::Org) the store belongs to.
  pub org:                OrgRecordId,
}

impl Model for Store {
  const TABLE_NAME: &'static str = STORE_TABLE_NAME;
  const UNIQUE_INDICES: &'static [(
    &'static str,
    crate::SlugFieldGetter<Self>,
  )] = &[];
  const INDICES: &'static [(&'static str, model::SlugFieldGetter<Self>)] = &[];

  fn id(&self) -> StoreRecordId { self.id }
}

/// The request to create a store.
#[derive(Clone, Debug)]
pub struct StoreCreateRequest {
  /// The store's nickname.
  pub nickname:           dvf::EntityNickname,
  /// The store's credentials.
  pub config:             StorageCredentials,
  /// The store's compression configuration.
  pub compression_config: dvf::CompressionConfig,
  /// The [`Org`](crate::Org) the store belongs to.
  pub org:                OrgRecordId,
}

impl From<StoreCreateRequest> for Store {
  fn from(req: StoreCreateRequest) -> Self {
    Self {
      id:                 Default::default(),
      nickname:           req.nickname,
      credentials:        req.config,
      compression_config: req.compression_config,
      org:                req.org,
    }
  }
}
