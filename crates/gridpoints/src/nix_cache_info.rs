use axum::response::IntoResponse;

pub async fn nix_cache_info() -> impl IntoResponse {
  "StoreDir: /nix/store\nWantMassQuery: 1\nPriority: 30"
}
