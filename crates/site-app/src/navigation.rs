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
pub fn next_url_hook() -> Memo<String> {
  let query = leptos_router::hooks::use_query_map();
  let current_url = leptos_router::hooks::use_url();

  // set it to the existing next url or the current URL escaped
  Memo::new(move |_| {
    query()
      .get("next")
      .unwrap_or(Url::escape(&url_to_full_path(&current_url())))
  })
}
