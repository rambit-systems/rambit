use meta_domain::MetaService;
use models::{EmailAddress, HumanName};
use mutate_domain::MutationService;

use super::*;

#[tokio::test]
async fn test_user_signup() {
  let service = AuthDomainService::new(
    MetaService::new_mock(),
    MutationService::new_mock(),
  );

  let name = HumanName::try_new("Test User 1").unwrap();
  let email = EmailAddress::try_new("test@example.com").unwrap();
  let creds = UserSubmittedAuthCredentials::Password {
    password: "hunter42".to_string(),
  };
  let user = service
    .user_signup(name, email.clone(), creds.clone())
    .await
    .unwrap();
  assert_eq!(user.email, email);

  let name = HumanName::try_new("Test User 2").unwrap();
  let user2 = service
    .user_signup(name, email.clone(), creds.clone())
    .await;
  assert!(matches!(user2, Err(CreateUserError::EmailAlreadyUsed(_))));
}

#[tokio::test]
async fn test_user_authenticate() {
  let service = AuthDomainService::new(
    MetaService::new_mock(),
    MutationService::new_mock(),
  );

  let name = HumanName::try_new("Test User 1").unwrap();
  let email = EmailAddress::try_new("test@example.com").unwrap();
  let creds = UserSubmittedAuthCredentials::Password {
    password: "hunter42".to_string(),
  };
  let user = service
    .user_signup(name, email.clone(), creds.clone())
    .await
    .unwrap();
  assert_eq!(user.email, email);

  let auth_user = service
    .user_authenticate(email.clone(), creds)
    .await
    .unwrap();
  assert_eq!(auth_user, Some(user));

  let wrong_email = EmailAddress::try_new("untest@example.com").unwrap();
  let creds = UserSubmittedAuthCredentials::Password {
    password: "hunter42".to_string(),
  };
  let auth_user = service.user_authenticate(wrong_email, creds).await.unwrap();
  assert_eq!(auth_user, None);
}
