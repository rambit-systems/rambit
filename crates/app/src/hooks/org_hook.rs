use models::{AuthUser, Entry, Org, PvOrg, RecordId};

use crate::APP_PREFIX;

/// A hook that provides data on an [`Org`].
#[derive(Clone)]
pub struct OrgHook {
  org:  PvOrg,
  user: AuthUser,
}

impl OrgHook {
  /// Creates a new [`OrgHook`]. Requires [`AuthUser`].
  pub fn new(org: Org, user: AuthUser) -> Self {
    Self {
      org: org.into(),
      user,
    }
  }

  #[allow(dead_code)]
  pub fn id(&self) -> RecordId<Org> { self.org.id }

  /// The canonical user-facing org name/descriptor.
  pub fn descriptor(&self) -> String {
    self
      .org
      .user_facing_title(&self.user)
      .unwrap_or("[unknown-org]".to_owned())
  }
}

/// A hook that provides URLs relating to an [`Org`].
#[derive(Clone)]
pub struct OrgUrlHook {
  id: RecordId<Org>,
}

impl OrgUrlHook {
  /// Creates a new [`OrgUrlHook`].
  pub fn new(id: RecordId<Org>) -> Self { Self { id } }

  pub fn id(&self) -> RecordId<Org> { self.id }

  /// The base URL for the org. No page exists here.
  fn base_url(&self) -> String {
    format!("{APP_PREFIX}/org/{id}", id = self.id)
  }

  /// The URL for the org's dashboard page, relative to the site root.
  pub fn dashboard_url(&self) -> String { format!("{}/dash", self.base_url()) }

  pub fn dashboard_entry_table_infill_url(&self) -> String {
    format!("{}/dash/entry_table", self.base_url())
  }

  pub fn dashboard_cache_table_infill_url(&self) -> String {
    format!("{}/dash/cache_table", self.base_url())
  }

  /// The URL for the org's "create cache" page, relative to the site root.
  pub fn create_cache_url(&self) -> String {
    format!("{}/create_cache", self.base_url())
  }

  pub fn create_cache_validation_url(&self) -> String {
    format!("{}/create_cache/validate", self.base_url())
  }

  pub fn create_cache_action_url(&self) -> String {
    format!("{}/create_cache/action", self.base_url())
  }

  /// The URL for the org's "create store" page, relative to the site root.
  pub fn create_store_url(&self) -> String {
    format!("{}/create_store", self.base_url())
  }

  /// The URL for the page of a given entry in the org, relative to the site
  /// root.
  pub fn entry_url(&self, entry_id: RecordId<Entry>) -> String {
    format!("{base}/entry/{entry_id}", base = self.base_url())
  }

  /// The URL for the org's setting page, relative to the site root.
  pub fn settings_url(&self) -> String {
    format!("{}/settings", self.base_url())
  }
}
