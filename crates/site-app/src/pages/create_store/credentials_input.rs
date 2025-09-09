use leptos::prelude::*;
use models::R2StorageCredentials;

use crate::{
  components::{InputField, InputIcon},
  reactive_utils::touched_input_bindings,
};

#[component]
pub fn CredentialsInput(
  signal: RwSignal<Option<R2StorageCredentials>>,
  show_hints: impl Fn() -> bool + Copy + Send + Sync + 'static,
) -> impl IntoView {
  let access_key = RwSignal::new(String::new());
  let secret_access_key = RwSignal::new(String::new());
  let bucket = RwSignal::new(String::new());
  let endpoint = RwSignal::new(String::new());
  let (read_access_key, write_access_key) = touched_input_bindings(access_key);
  let (read_secret_access_key, write_secret_access_key) =
    touched_input_bindings(secret_access_key);
  let (read_bucket, write_bucket) = touched_input_bindings(bucket);
  let (read_endpoint, write_endpoint) = touched_input_bindings(endpoint);

  let access_key_error = Signal::derive(move || {
    access_key
      .get()
      .is_empty()
      .then_some("Access key required.".to_owned())
  });
  let secret_access_key_error = Signal::derive(move || {
    secret_access_key
      .get()
      .is_empty()
      .then_some("Secret access key required.".to_owned())
  });
  let bucket_error = Signal::derive(move || {
    [
      bucket
        .get()
        .is_empty()
        .then_some("Bucket required.".to_owned()),
      (!bucket.get().is_ascii()).then_some("Bucket must be ASCII.".to_owned()),
    ]
    .into_iter()
    .flatten()
    .next()
  });
  let endpoint_error = Signal::derive(move || {
    endpoint
      .get()
      .is_empty()
      .then_some("Endpoint required.".to_owned())
  });

  let error_to_error_hint = move |e: Signal<Option<String>>| {
    MaybeProp::derive(move || show_hints().then_some(e()).flatten())
  };
  let access_key_error_hint = error_to_error_hint(access_key_error);
  let secret_access_key_error_hint =
    error_to_error_hint(secret_access_key_error);
  let bucket_error_hint = error_to_error_hint(bucket_error);
  let endpoint_error_hint = error_to_error_hint(endpoint_error);

  let final_product = Memo::new(move |_| {
    if access_key_error().is_some()
      || secret_access_key_error().is_some()
      || bucket_error().is_some()
      || endpoint_error().is_some()
    {
      return None;
    }

    Some(R2StorageCredentials::Default {
      access_key:        access_key(),
      secret_access_key: secret_access_key(),
      endpoint:          endpoint(),
      bucket:            bucket(),
    })
  });

  // synchronize inputs to signal
  Effect::watch(
    final_product,
    move |final_product, _, _| signal.set(final_product.clone()),
    false,
  );

  view! {
    <div class="flex flex-col gap-2">
      <InputField
        id="access_key" label_text="Access Key" input_type="password" placeholder=""
        input_signal=read_access_key output_signal=write_access_key
        error_hint=access_key_error_hint warn_hint={ MaybeProp::derive(|| None) }
        before={InputIcon::Key}
      />
      <InputField
        id="secret_access_key" label_text="Secret Access Key" input_type="password" placeholder=""
        input_signal=read_secret_access_key output_signal=write_secret_access_key
        error_hint=secret_access_key_error_hint warn_hint={ MaybeProp::derive(|| None) }
        before={InputIcon::Key}
      />
      <InputField
        id="bucket" label_text="Bucket" input_type="text" placeholder=""
        input_signal=read_bucket output_signal=write_bucket
        error_hint=bucket_error_hint warn_hint={ MaybeProp::derive(|| None) }
        before={InputIcon::ArchiveBox}
      />
      <InputField
        id="endpoint" label_text="Endpoint" input_type="text" placeholder=""
        input_signal=read_endpoint output_signal=write_endpoint
        error_hint=endpoint_error_hint warn_hint={ MaybeProp::derive(|| None) }
        before={InputIcon::GlobeAlt}
      />
    </div>
  }
}
