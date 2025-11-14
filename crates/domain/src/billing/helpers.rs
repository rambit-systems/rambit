use models::{Org, PaddleSubscription, RecordId};
use tracing::{instrument, warn};

#[instrument(skip(sub))]
pub(super) fn associated_org_id_from_subscription(
  sub: &PaddleSubscription,
) -> Option<RecordId<Org>> {
  let Some(data) = sub.custom_data.as_ref() else {
    warn!("subscription {id} had no custom data", id = sub.id);
    return None;
  };

  let Some(object) = data.as_object() else {
    warn!(
      ?data,
      "subscription {id} had custom data that was not an object",
      id = sub.id
    );
    return None;
  };

  let Some(value) = object.get("org_id") else {
    warn!(
      "subscription {id} custom data missing for key `org_id`",
      id = sub.id
    );
    return None;
  };

  let Some(string) = value.as_str() else {
    warn!(
      "subcsription {id} custom data for key `org_id` was not a string",
      id = sub.id
    );
    return None;
  };

  string
    .parse()
    .map_err(|e| {
      warn!(
        string,
        err = ?e,
        "failed to parse custom data for key `org_id` on subscription {id} as \
         record id",
        id = sub.id
      );
    })
    .ok()
}
