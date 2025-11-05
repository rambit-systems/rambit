use std::str::FromStr;

use belt::Belt;
use bytes::Bytes;
use models::{EntityName, NarDeriverData, RecordId, StorePath};

use crate::{DomainService, download::DownloadRequest, upload::UploadRequest};

#[tokio::test]
async fn test_download() {
  let pds = DomainService::mock_domain().await;

  let input_bytes = Bytes::from_static(include_bytes!(
    "../../../owl/test/ky2wzr68im63ibgzksbsar19iyk861x6-bat-0.25.0"
  ));
  let nar_contents = Belt::new_from_bytes(input_bytes.clone());

  let user_id = RecordId::from_str("01JXGXV4R6VCZWQ2DAYDWR1VXD").unwrap();
  let cache_name = EntityName::new("aaron");
  let target_store = EntityName::new("albert");
  let store_path = "/nix/store/ky2wzr68im63ibgzksbsar19iyk861x6-bat-0.25.0";
  let store_path =
    StorePath::from_absolute_path(store_path.as_bytes()).unwrap();

  let deriver_path =
    "/nix/store/4yz8qa58nmysad5w88rgdhq15rkssqr6-bat-0.25.0.drv".to_string();
  let deriver_path = StorePath::from_absolute_path(
    deriver_path.strip_suffix(".drv").unwrap().as_bytes(),
  )
  .unwrap();
  let deriver_data = NarDeriverData {
    system:  Some("aarch64-linux".to_string()),
    deriver: Some(deriver_path),
  };

  let req = UploadRequest {
    nar_contents,
    auth: user_id,
    caches: vec![cache_name.clone()],
    target_store,
    store_path: store_path.clone(),
    deriver_data,
  };

  let plan = pds.plan_upload(req).await.expect("failed to plan upload");
  let resp = pds
    .execute_upload(plan)
    .await
    .expect("failed to execute upload");

  let _entry = pds
    .meta()
    .fetch_entry_by_id(resp.entry_id)
    .await
    .expect("failed to find entry")
    .expect("failed to find entry");
  // upload complete

  let download_req = DownloadRequest {
    auth: None,
    cache_name,
    store_path,
  };

  let download_plan = pds
    .plan_download(download_req)
    .await
    .expect("failed to plan download");
  let download_resp = pds
    .execute_download(download_plan)
    .await
    .expect("failed to execute download");

  let downloaded_data = download_resp
    .data
    .collect_bytes()
    .await
    .expect("failed to collect bytes from belt");
  assert_eq!(input_bytes, downloaded_data);
}
