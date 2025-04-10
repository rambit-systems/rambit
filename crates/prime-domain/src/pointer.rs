use miette::Result;
use models::{
  CacheRecordId, EntryRecordId, LaxSlug, StoreRecordId, StrictSlug,
  TokenRecordId, TokenSecret,
};
use mollusk::FetchPathError;
use repos::{
  belt::Belt,
  db::{FetchModelByIndexError, FetchModelError},
  Cache, Entry, StorageReadError, StorageWriteError, Store, Token,
};

use crate::{
  CreateEntryError, PrimeDomainService, ReadFromEntryError, TokenVerifyError,
};

// impl for smart pointers
#[async_trait::async_trait]
impl<T, I> PrimeDomainService for T
where
  T: std::ops::Deref<Target = I> + Send + Sync + 'static,
  I: PrimeDomainService + ?Sized,
{
  async fn fetch_cache_by_id(
    &self,
    id: CacheRecordId,
  ) -> Result<Option<Cache>, FetchModelError> {
    self.deref().fetch_cache_by_id(id).await
  }
  async fn fetch_entry_by_id(
    &self,
    id: EntryRecordId,
  ) -> Result<Option<Entry>, FetchModelError> {
    self.deref().fetch_entry_by_id(id).await
  }
  async fn fetch_store_by_id(
    &self,
    id: StoreRecordId,
  ) -> Result<Option<Store>, FetchModelError> {
    self.deref().fetch_store_by_id(id).await
  }
  async fn fetch_token_by_id(
    &self,
    id: TokenRecordId,
  ) -> Result<Option<Token>, FetchModelError> {
    self.deref().fetch_token_by_id(id).await
  }
  async fn enumerate_caches(&self) -> Result<Vec<Cache>> {
    self.deref().enumerate_caches().await
  }
  async fn enumerate_entries(&self) -> Result<Vec<Entry>> {
    self.deref().enumerate_entries().await
  }
  async fn enumerate_stores(&self) -> Result<Vec<Store>> {
    self.deref().enumerate_stores().await
  }
  async fn enumerate_tokens(&self) -> Result<Vec<Token>> {
    self.deref().enumerate_tokens().await
  }

  async fn find_cache_by_name(
    &self,
    name: StrictSlug,
  ) -> Result<Option<Cache>, FetchModelByIndexError> {
    self.deref().find_cache_by_name(name).await
  }
  async fn find_entry_by_id_and_path(
    &self,
    cache_id: CacheRecordId,
    path: models::LaxSlug,
  ) -> Result<Option<Entry>, FetchModelByIndexError> {
    self.deref().find_entry_by_id_and_path(cache_id, path).await
  }
  async fn verify_token_id_and_secret(
    &self,
    id: TokenRecordId,
    secret: models::TokenSecret,
  ) -> Result<Token, TokenVerifyError> {
    self.deref().verify_token_id_and_secret(id, secret).await
  }

  async fn create_entry(
    &self,
    owning_cache: CacheRecordId,
    path: LaxSlug,
    data: Belt,
  ) -> Result<Entry, CreateEntryError> {
    self.deref().create_entry(owning_cache, path, data).await
  }
  async fn read_from_entry(
    &self,
    entry_id: EntryRecordId,
  ) -> Result<Belt, ReadFromEntryError> {
    self.deref().read_from_entry(entry_id).await
  }

  async fn read_from_temp_storage(
    &self,
    path: models::TempStoragePath,
  ) -> Result<Belt, StorageReadError> {
    self.deref().read_from_temp_storage(path).await
  }
  async fn write_to_temp_storage(
    &self,
    data: Belt,
  ) -> Result<models::TempStoragePath, StorageWriteError> {
    self.deref().write_to_temp_storage(data).await
  }

  async fn fetch_path(
    &self,
    cache_name: StrictSlug,
    token_id: Option<TokenRecordId>,
    token_secret: Option<TokenSecret>,
    path: LaxSlug,
  ) -> Result<Belt, FetchPathError> {
    self
      .deref()
      .fetch_path(cache_name, token_id, token_secret, path)
      .await
  }
}
