use std::{path::PathBuf, str::FromStr};

use miette::Result;
use models::{
  Cache, CachePermissionType, CacheRecordId, EntityName, EntityNickname,
  HumanName, LocalStorageCredentials, Org, Permission, PermissionSet, RecordId,
  StorageCredentials, Store, StoreRecordId, StrictSlug, Token, TokenRecordId,
  TokenSecret, User, UserRecordId,
};

use crate::Database;

/// A migrator for the database.
pub struct Migrator {
  org_db:   Database<Org>,
  user_db:  Database<User>,
  store_db: Database<Store>,
  cache_db: Database<Cache>,
  token_db: Database<Token>,
}

impl Migrator {
  /// Creates a new migrator.
  pub fn new(
    org_db: Database<Org>,
    user_db: Database<User>,
    store_db: Database<Store>,
    cache_db: Database<Cache>,
    token_db: Database<Token>,
  ) -> Self {
    Self {
      org_db,
      user_db,
      store_db,
      cache_db,
      token_db,
    }
  }

  /// Applies test data to the database.
  pub async fn migrate(&self) -> Result<()> {
    let org = Org {
      id:   RecordId::<Org>::from_str("01J53FHN8TQXTQ2JEHNX56GCTN").unwrap(),
      name: EntityName::new(StrictSlug::confident("dev-org")),
    };

    let user = User {
      id:   UserRecordId::from_str("01J53N6ARQGFTBQ41T25TAJ949").unwrap(),
      name: HumanName::try_new("John Lewis".to_string()).unwrap(),
      org:  org.id,
    };

    let local_file_store = Store {
      id:                 StoreRecordId::from_str("01J53YYCCJW4B4QBM1CG0CHAMP")
        .unwrap(),
      nickname:           EntityNickname::new(StrictSlug::confident(
        "local-file-store",
      )),
      credentials:        StorageCredentials::Local(LocalStorageCredentials(
        PathBuf::from_str("/tmp/local-store").unwrap(),
      )),
      compression_config: models::CompressionConfig::new(Some(
        models::CompressionAlgorithm::Zstd,
      )),
      org:                org.id,
    };

    let albert_cache = Cache {
      id:         CacheRecordId::from_str("01J799MSHXPPY5RJ8KGHVR9GWQ")
        .unwrap(),
      name:       EntityName::new(StrictSlug::confident("albert")),
      visibility: models::Visibility::Private,
      store:      local_file_store.id,
      org:        org.id,
    };

    let byron_cache = Cache {
      id:         CacheRecordId::from_str("01JFTEBFJ55TVWC7Z4BMPBX8AP")
        .unwrap(),
      name:       EntityName::new(StrictSlug::confident("byron")),
      visibility: models::Visibility::Public,
      store:      local_file_store.id,
      org:        org.id,
    };

    let omnitoken_token = Token {
      id:       TokenRecordId::from_str("01J53ZA38PS1P5KWCE4FMG58F0").unwrap(),
      nickname: EntityNickname::new(StrictSlug::confident("omnitoken")),
      secret:   TokenSecret::new(StrictSlug::confident(
        "zvka5d29dgvpujdyqa6ftnkei02i-qm1n-fjzuqfbyrq7avxbzi6ma8flxsuwe4l",
      )),
      perms:    PermissionSet(
        vec![
          Permission::CachePermission {
            cache_id:   albert_cache.id,
            permission: CachePermissionType::Read,
          },
          Permission::CachePermission {
            cache_id:   albert_cache.id,
            permission: CachePermissionType::Write,
          },
          Permission::CachePermission {
            cache_id:   byron_cache.id,
            permission: CachePermissionType::Read,
          },
          Permission::CachePermission {
            cache_id:   byron_cache.id,
            permission: CachePermissionType::Write,
          },
        ]
        .into_iter()
        .collect(),
      ),
      owner:    user.id,
      org:      org.id,
    };

    self.org_db.create_model(org).await?;
    self.user_db.create_model(user).await?;
    self.store_db.create_model(local_file_store).await?;
    self.cache_db.create_model(albert_cache).await?;
    self.cache_db.create_model(byron_cache).await?;
    self.token_db.create_model(omnitoken_token).await?;

    Ok(())
  }
}
