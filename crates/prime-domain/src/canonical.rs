use std::{path::PathBuf, str::FromStr};

pub use hex;
use hex::health;
use miette::Result;
pub use models;
use models::{
  Cache, CacheRecordId, Entry, EntryCreateRequest, EntryRecordId, LaxSlug,
  Store, StoreRecordId, StrictSlug, Token, TokenRecordId, TokenSecret,
};
use mollusk::{
  FetchPathError, InternalError, MissingPathError, NonExistentCacheError,
  UnauthenticatedCacheAccessError, UnauthorizedCacheAccessError,
};
pub use repos::{self, StorageReadError, StorageWriteError};
use repos::{
  belt::{self, Belt},
  db::{FetchModelByIndexError, FetchModelError},
  CacheRepository, EntryRepository, StoreRepository, TempStorageRepository,
  TokenRepository, UserStorageClient,
};
use tracing::instrument;

fn dvf_comp_to_belt_comp(
  comp: Option<models::CompressionAlgorithm>,
) -> Option<belt::CompressionAlgorithm> {
  comp
    .map(|models::CompressionAlgorithm::Zstd| belt::CompressionAlgorithm::Zstd)
}

use crate::{
  CreateEntryError, PrimeDomainService, ReadFromEntryError, TokenVerifyError,
};

/// The canonical implementation of [`PrimeDomainService`].
pub struct PrimeDomainServiceCanonical<
  CR: CacheRepository,
  ER: EntryRepository,
  SR: StoreRepository,
  TR: TokenRepository,
  TSR: TempStorageRepository,
  USR: repos::UserStorageRepository,
> {
  cache_repo:        CR,
  entry_repo:        ER,
  store_repo:        SR,
  token_repo:        TR,
  temp_storage_repo: TSR,
  user_storage_repo: USR,
}

impl<CR, ER, SR, TR, TSR, USR>
  PrimeDomainServiceCanonical<CR, ER, SR, TR, TSR, USR>
where
  CR: CacheRepository,
  ER: EntryRepository,
  SR: StoreRepository,
  TR: TokenRepository,
  TSR: TempStorageRepository,
  USR: repos::UserStorageRepository,
{
  /// Create a new instance of the canonical prime domain service.
  pub fn new(
    cache_repo: CR,
    entry_repo: ER,
    store_repo: SR,
    token_repo: TR,
    temp_storage_repo: TSR,
    user_storage_repo: USR,
  ) -> Self {
    tracing::info!("creating new `PrimeDomainServiceCanonical` instance");
    Self {
      cache_repo,
      entry_repo,
      store_repo,
      token_repo,
      temp_storage_repo,
      user_storage_repo,
    }
  }

  /// Write data to a store, respecting compression settings.
  async fn write_to_store(
    &self,
    store_id: StoreRecordId,
    path: models::LaxSlug,
    data: Belt,
  ) -> Result<models::CompressionStatus, crate::WriteToStoreError> {
    // fetch the store
    let store = self
      .fetch_store_by_id(store_id)
      .await
      .map_err(crate::WriteToStoreError::FetchError)?
      .ok_or_else(|| crate::WriteToStoreError::StoreNotFound(store_id))?;

    // count the uncompressed size
    let uncompressed_counter = data.counter();

    // check what compression algorithm is configured in the store
    let algorithm = store.compression_config.algorithm();

    // adapt to compress the data if needed
    let data = match algorithm {
      Some(models::CompressionAlgorithm::Zstd) => {
        data.adapt_to_comp(belt::CompressionAlgorithm::Zstd)
      }
      None => data,
    };

    // count the compressed size
    let compressed_counter = data.counter();

    // get the user storage client
    let client = self
      .user_storage_repo
      .connect_to_user_storage(store.credentials.clone())
      .await
      .map_err(crate::WriteToStoreError::StorageConnectionError)?;

    // write the data to the store
    let path = PathBuf::from_str(path.as_ref()).unwrap();
    let _ = client
      .write(&path, data)
      .await
      .map_err(crate::WriteToStoreError::StorageWriteError)?;

    // get the sizes
    let uncompressed_file_size = uncompressed_counter.current();
    let compressed_file_size = compressed_counter.current();

    // return the compression status
    let c_status = match algorithm {
      Some(algorithm) => models::CompressionStatus::Compressed {
        compressed_size: models::FileSize::new(compressed_file_size),
        uncompressed_size: models::FileSize::new(uncompressed_file_size),
        algorithm,
      },
      None => models::CompressionStatus::Uncompressed {
        size: models::FileSize::new(uncompressed_file_size),
      },
    };

    Ok(c_status)
  }
}

#[async_trait::async_trait]
impl<CR, ER, SR, TR, TSR, USR> PrimeDomainService
  for PrimeDomainServiceCanonical<CR, ER, SR, TR, TSR, USR>
where
  CR: CacheRepository,
  ER: EntryRepository,
  SR: StoreRepository,
  TR: TokenRepository,
  TSR: TempStorageRepository,
  USR: repos::UserStorageRepository,
{
  async fn fetch_cache_by_id(
    &self,
    id: CacheRecordId,
  ) -> Result<Option<Cache>, FetchModelError> {
    self.cache_repo.fetch_model_by_id(id).await
  }
  async fn fetch_entry_by_id(
    &self,
    id: EntryRecordId,
  ) -> Result<Option<Entry>, FetchModelError> {
    self.entry_repo.fetch_model_by_id(id).await
  }
  async fn fetch_store_by_id(
    &self,
    id: StoreRecordId,
  ) -> Result<Option<Store>, FetchModelError> {
    self.store_repo.fetch_model_by_id(id).await
  }
  async fn fetch_token_by_id(
    &self,
    id: TokenRecordId,
  ) -> Result<Option<Token>, FetchModelError> {
    self.token_repo.fetch_model_by_id(id).await
  }
  async fn enumerate_caches(&self) -> Result<Vec<Cache>> {
    self.cache_repo.enumerate_models().await
  }
  async fn enumerate_entries(&self) -> Result<Vec<Entry>> {
    self.entry_repo.enumerate_models().await
  }
  async fn enumerate_stores(&self) -> Result<Vec<Store>> {
    self.store_repo.enumerate_models().await
  }
  async fn enumerate_tokens(&self) -> Result<Vec<Token>> {
    self.token_repo.enumerate_models().await
  }

  async fn find_cache_by_name(
    &self,
    name: StrictSlug,
  ) -> Result<Option<Cache>, FetchModelByIndexError> {
    self.cache_repo.find_by_name(name).await
  }
  async fn find_entry_by_id_and_path(
    &self,
    cache_id: CacheRecordId,
    path: models::LaxSlug,
  ) -> Result<Option<Entry>, FetchModelByIndexError> {
    self
      .entry_repo
      .find_entry_by_id_and_path(cache_id, path)
      .await
  }
  async fn verify_token_id_and_secret(
    &self,
    id: TokenRecordId,
    secret: models::TokenSecret,
  ) -> Result<Token, TokenVerifyError> {
    let token = self
      .token_repo
      .fetch_model_by_id(id)
      .await
      .map_err(TokenVerifyError::FetchError)?
      .ok_or(TokenVerifyError::IdNotFound)?;

    if token.secret != secret {
      return Err(TokenVerifyError::SecretMismatch);
    }
    Ok(token)
  }

  async fn create_entry(
    &self,
    owning_cache: CacheRecordId,
    path: LaxSlug,
    data: Belt,
  ) -> Result<Entry, CreateEntryError> {
    // check if the entry already exists
    let existing_entry = self
      .find_entry_by_id_and_path(owning_cache, path.clone())
      .await
      .map_err(CreateEntryError::FetchModelByIndexError)?;
    if existing_entry.is_some() {
      return Err(CreateEntryError::EntryAlreadyExists);
    }

    let cache = self
      .fetch_cache_by_id(owning_cache)
      .await
      .map_err(CreateEntryError::FetchModelError)?
      .ok_or(CreateEntryError::CacheNotFound(owning_cache))?;

    let c_status = self.write_to_store(cache.store, path.clone(), data).await?;

    let entry_cr = EntryCreateRequest {
      path,
      c_status,
      cache: owning_cache,
      org: cache.org,
    };

    let entry = self
      .entry_repo
      .create_model(entry_cr)
      .await
      .map_err(CreateEntryError::CreateError)?;

    Ok(entry)
  }
  async fn read_from_entry(
    &self,
    entry_id: EntryRecordId,
  ) -> Result<Belt, ReadFromEntryError> {
    let entry = self
      .fetch_entry_by_id(entry_id)
      .await
      .map_err(ReadFromEntryError::FetchModelError)?
      .ok_or_else(|| ReadFromEntryError::EntryNotFound(entry_id))?;

    let cache = self
      .fetch_cache_by_id(entry.cache)
      .await
      .map_err(ReadFromEntryError::FetchModelError)?
      .ok_or_else(|| {
        ReadFromEntryError::DataIntegrityError(miette::miette!(
          "entry references non-existent cache: {}",
          entry.cache
        ))
      })?;

    let store = self
      .fetch_store_by_id(cache.store)
      .await
      .map_err(ReadFromEntryError::FetchModelError)?
      .ok_or_else(|| {
        ReadFromEntryError::DataIntegrityError(miette::miette!(
          "cache references non-existent store: {}",
          cache.store
        ))
      })?;

    // check what compression algorithm is configured in the store
    let algorithm = entry.c_status.algorithm();

    // get the user storage client
    let client = self
      .user_storage_repo
      .connect_to_user_storage(store.credentials.clone())
      .await
      .map_err(ReadFromEntryError::StorageConnectionError)?;

    let path = PathBuf::from_str(entry.path.as_ref()).unwrap();
    let reader = client
      .read(&path)
      .await
      .map_err(ReadFromEntryError::StorageReadError)?;

    let reader = reader.set_declared_comp(dvf_comp_to_belt_comp(algorithm));

    Ok(reader)
  }

  async fn read_from_temp_storage(
    &self,
    path: models::TempStoragePath,
  ) -> Result<Belt, StorageReadError> {
    self.temp_storage_repo.read(path).await
  }
  async fn write_to_temp_storage(
    &self,
    data: Belt,
  ) -> Result<models::TempStoragePath, StorageWriteError> {
    self.temp_storage_repo.store(data).await
  }

  async fn fetch_path(
    &self,
    cache_name: StrictSlug,
    token_id: Option<TokenRecordId>,
    token_secret: Option<TokenSecret>,
    path: LaxSlug,
  ) -> Result<Belt, mollusk::FetchPathError> {
    let cache = self
      .find_cache_by_name(cache_name.clone())
      .await
      .map_err(|e| {
        FetchPathError::InternalError(InternalError(format!("{e:?}")))
      })?
      .ok_or(NonExistentCacheError(cache_name.to_string()))?;

    if matches!(cache.visibility, models::Visibility::Private) {
      // if the store is not public, we must have a token
      let token_id = token_id
        .ok_or(UnauthenticatedCacheAccessError(cache_name.to_string()))?;
      let token_secret = token_secret
        .ok_or(UnauthenticatedCacheAccessError(cache_name.to_string()))?;

      let required_permission = models::Permission::CachePermission {
        cache_id:   cache.id,
        permission: models::CachePermissionType::Read,
      };
      let required_permission_set =
        models::PermissionSet::from_iter(vec![required_permission.clone()]);

      let token = self
        .verify_token_id_and_secret(token_id, token_secret.clone())
        .await
        .map_err(|e| {
          FetchPathError::InternalError(InternalError(format!("{e:?}")))
        })?;
      let authorized = token.authorized(&required_permission_set);

      if !authorized {
        Err(UnauthorizedCacheAccessError {
          cache_name: cache.name.clone().into_inner().into_inner(),
          permission: models::CachePermissionType::Read,
        })?;
      }
    }

    let entry = self
      .find_entry_by_id_and_path(cache.id, path.clone())
      .await
      .map_err(|e| InternalError(format!("{e:?}")))?
      .ok_or(MissingPathError {
        path: path.to_string(),
      })?;

    let belt = self.read_from_entry(entry.id).await.map_err(|e| {
      FetchPathError::InternalError(InternalError(format!("{e:?}")))
    })?;

    Ok(belt)
  }
}

#[async_trait::async_trait]
impl<CR, ER, SR, TR, TSR, USR> health::HealthReporter
  for PrimeDomainServiceCanonical<CR, ER, SR, TR, TSR, USR>
where
  CR: CacheRepository,
  ER: EntryRepository,
  SR: StoreRepository,
  TR: TokenRepository,
  TSR: TempStorageRepository,
  USR: repos::UserStorageRepository,
{
  fn name(&self) -> &'static str { stringify!(PrimeDomainServiceCanonical) }
  #[instrument(skip(self))]
  async fn health_check(&self) -> health::ComponentHealth {
    health::AdditiveComponentHealth::from_futures(vec![
      self.cache_repo.health_report(),
      self.entry_repo.health_report(),
      self.store_repo.health_report(),
      self.token_repo.health_report(),
      self.temp_storage_repo.health_report(),
      self.user_storage_repo.health_report(),
    ])
    .await
    .into()
  }
}
