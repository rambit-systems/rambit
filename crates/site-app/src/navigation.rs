use leptos::{logging, prelude::*, web_sys};
use leptos_router::location::Url;

// taken from https://github.com/leptos-rs/leptos/blob/2ee4444bb44310e73e908b98ccd2b353f534da01/router/src/location/mod.rs#L87-L100
/// Constructs the "full path" (relative to origin, starting from "/") from a
/// [`Url`].
pub fn url_to_full_path(url: &Url) -> String {
  let mut path = url.path().to_string();
  if !url.search().is_empty() {
    path.push('?');
    path.push_str(url.search());
  }
  // if !url.hash().is_empty() {
  //   if !url.hash().starts_with('#') {
  //     path.push('#');
  //   }
  //   path.push_str(url.hash());
  // }
  path
}

/// Reload the page.
#[expect(dead_code)]
pub fn reload() {
  let Some(window) = web_sys::window() else {
    logging::error!("failed to get window");
    return;
  };
  let result = window.location().reload();
  if let Err(e) = result {
    logging::error!("failed to reload: {:?}", e);
  }
}

/// Navigate to a new page.
pub fn navigate_to(path: &str) {
  logging::log!("navigating to: {}", path);
  let Some(window) = web_sys::window() else {
    logging::error!("failed to get window");
    return;
  };
  let result = window.location().set_href(path);
  if let Err(e) = result {
    logging::error!("failed to navigate: {:?}", e);
  }
}

/// Gets the next URL if it's already set or sets it to the current page.
pub fn next_url_string_hook() -> Signal<String> {
  #[cfg(not(feature = "ssr"))]
  use leptos_router::location::LocationProvider;

  #[cfg(feature = "ssr")]
  let current_url = leptos_router::hooks::use_url()();
  #[cfg(not(feature = "ssr"))]
  let current_url = leptos_router::location::BrowserUrl::current()
    .expect("failed to get current browser url");

  Signal::stored(
    current_url
      .search_params()
      .clone()
      .get("next")
      .unwrap_or(url_to_full_path(&current_url)),
  )
}

/// Url-enccodes the next URL if it's already set or sets it to the current
/// page. Suitable for propagating the next URL by setting the param to this
/// value in links.
pub fn next_url_encoded_hook() -> Signal<String> {
  let next_url = next_url_string_hook();
  Signal::derive(move || {
    let next_url = next_url();
    Url::escape(&next_url)
  })
}
