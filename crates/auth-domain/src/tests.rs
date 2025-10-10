use models::dvf::{EmailAddress, HumanName};

use super::*;

#[tokio::test]
async fn test_user_signup() {
  let org_repo = Database::new_mock();
  let user_repo = Database::new_mock();
  let service = AuthDomainService::new(org_repo, user_repo);

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

  dbg!(&service);

  let name = HumanName::try_new("Test User 2").unwrap();
  let user2 = service
    .user_signup(name, email.clone(), creds.clone())
    .await;
  assert!(matches!(user2, Err(CreateUserError::EmailAlreadyUsed(_))));
}

#[tokio::test]
async fn test_user_authenticate() {
  let org_repo = Database::new_mock();
  let user_repo = Database::new_mock();
  let service = AuthDomainService::new(org_repo, user_repo);

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
