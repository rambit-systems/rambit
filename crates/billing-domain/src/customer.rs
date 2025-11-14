use std::collections::HashMap;

use miette::{Context, IntoDiagnostic, Report};
use models::{EmailAddress, PaddleCustomerId, RecordId, User};
use paddle_rust_sdk::{error::PaddleApiError, response::ErrorResponse};
use tracing::instrument;

use super::BillingService;

impl BillingService {
  /// Creates a new customer if a customer with the given email does not
  /// already exist. Otherwise, update the ID and name of the customer
  /// whose email matches.
  #[instrument(skip(self))]
  pub async fn upsert_customer(
    &self,
    org_id: RecordId<User>,
    name: &str,
    email: &EmailAddress,
  ) -> Result<PaddleCustomerId, Report> {
    // attempt to just create a user
    let mut req = self.paddle_client.customer_create(email.as_ref());
    req
      .name(name)
      .custom_data(HashMap::from_iter([("id".to_owned(), org_id.to_string())]));
    let create_result = req.send().await;

    // short circuit if it worked
    let err = match create_result {
      Ok(customer) => return Ok(customer.data.id),
      Err(e) => e,
    };

    // extract the ID if it's a duplicate customer error
    let id = match err {
      paddle_rust_sdk::Error::PaddleApi(ErrorResponse {
        error: PaddleApiError { code, detail, .. },
        ..
      }) if code == "customer_already_exists" => detail
        .split(" ")
        .last()
        .ok_or(miette::miette!(
          "could not find customer ID in duplicate customer error: {detail:?}"
        ))?
        .to_owned(),
      e => {
        return Err(
          Report::from_err(e)
            .context("unknown paddle error in attempted customer creation"),
        );
      }
    };

    // update the customer name and Rambit ID, and activate if archived
    let mut update_req = self.paddle_client.customer_update(&*id);
    update_req
      .name(name)
      .custom_data(HashMap::from_iter([("id".to_owned(), org_id.to_string())]))
      .status(paddle_rust_sdk::enums::Status::Active);

    let customer = update_req
      .send()
      .await
      .into_diagnostic()
      .context("failed to update paddle customer")?
      .data;

    Ok(customer.id)
  }
}
