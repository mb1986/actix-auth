use actix_web::dev::{Payload, Service};
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::test::{block_on, init_service, TestRequest};
use actix_web::{App, Error, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};

use actix_auth::{AuthConfig, AuthService};

#[derive(Serialize, Deserialize)]
struct UserId(String);

struct User;

struct Connection;

impl FromRequest for Connection {
    type Error = Error;
    type Future = Result<Self, Self::Error>;
    type Config = ();

    fn from_request(_req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        Ok(Connection)
    }
}

impl AuthService for User {
    type Context = Connection;
    type UserData = UserId;
    type Error = Error;
    type Future = Result<Option<Self::UserData>, Self::Error>;

    fn authenticate(username: &str, password: &str, _: &Self::Context) -> Self::Future {
        if username == "test_user" && password == "test_pass" {
            Ok(Some(UserId("test_user_id".to_string())))
        } else {
            Ok(None)
        }
    }
}

#[test]
fn wrong_credentials_test() {
    let mut app = init_service(
        App::new().configure(
            AuthConfig::<User>::new()
                .path("auth")
                .session_ttl(259200)
                .configure(),
        ),
    );

    let payload = r#"{"username":"wrong_user","password":"wrong_pass","type":"user"}"#;
    let req = TestRequest::post()
        .uri("/auth")
        .set(ContentType::json())
        .set_payload(payload)
        .to_request();
    let res = block_on(app.call(req)).unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
