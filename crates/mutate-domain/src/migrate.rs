use std::str::FromStr;

use miette::{Context, IntoDiagnostic, Result};
use models::{
  Cache, EmailAddress, EntityName, HumanName, MemoryStorageCredentials, Org,
  OrgIdent, RecordId, StorageCredentials, Store, StoreConfiguration, User,
  Visibility,
};

use super::MutationService;

impl MutationService {
  /// Add test data to databases.
  pub async fn migrate_test_data(&self) -> Result<()> {
    let user_id = RecordId::from_str("01JXGXV4R6VCZWQ2DAYDWR1VXD").unwrap();

    let personal_org = Org {
      id:        RecordId::from_str("01K202SRBQMRM29MMSTJTMSJVD").unwrap(),
      org_ident: OrgIdent::UserOrg(user_id),
    };
    self
      .org_repo
      .insert(&personal_org)
      .await
      .into_diagnostic()
      .context("failed to create org")?;

    let federation = Org {
      id:        RecordId::from_str("01JXGXSB69BDHNFTSVG2EPW2M3").unwrap(),
      org_ident: OrgIdent::Named(EntityName::new("the-federation")),
    };
    self
      .org_repo
      .insert(&federation)
      .await
      .into_diagnostic()
      .context("failed to create org")?;

    let user = User {
      id:               user_id,
      personal_org:     personal_org.id,
      orgs:             vec![federation.id],
      email:            EmailAddress::try_new("jpicard@federation.gov")
        .unwrap(),
      name:             HumanName::try_new("Jean-Luc Picard")
        .expect("failed to create name"),
      name_abbr:        User::abbreviate_name(
        HumanName::try_new("Jean-Luc Picard").expect("failed to create name"),
      ),
      auth:             models::UserAuthCredentials::Password {
        // hash for password `password`
        password_hash: models::PasswordHash(
          "$argon2id$v=19$m=16,t=2,p=1$dGhpc2lzYXNhbHQ$dahcDJkLouoYfTwtXjg67Q"
            .to_string(),
        ),
      },
      active_org_index: 1,
    };

    self
      .user_repo
      .insert(&user)
      .await
      .into_diagnostic()
      .context("failed to create user")?;

    let albert_store = Store {
      id:          RecordId::from_str("01JXGXVF0MVQNGRM565YHM20BC").unwrap(),
      org:         federation.id,
      credentials: StorageCredentials::Memory(MemoryStorageCredentials),
      config:      StoreConfiguration {},
      name:        EntityName::new("albert"),
    };

    self
      .store_repo
      .insert(&albert_store)
      .await
      .into_diagnostic()
      .context("failed to create store")?;

    let aaron_cache = Cache {
      id:         RecordId::from_str("01JXGXVVE6J16590YJT3SP2P6M").unwrap(),
      org:        federation.id,
      name:       EntityName::new("aaron"),
      visibility: Visibility::Public,
    };

    self
      .cache_repo
      .insert(&aaron_cache)
      .await
      .into_diagnostic()
      .context("failed to create cache")?;

    Ok(())
  }
}
