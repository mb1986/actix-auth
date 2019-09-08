#![cfg(feature = "totp")]

use std::time::{SystemTime, UNIX_EPOCH};

use actix_session::CookieSession;
use actix_web::dev::{Payload, Service};
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::test::{block_on, init_service, TestRequest};
use actix_web::{App, Error, FromRequest, HttpRequest};
use otpauth::TOTP;
use serde::{Deserialize, Serialize};

use actix_auth::{AuthConfig, AuthData, AuthService};

const USER_TOTP_SECRET: &str = "user_totp_secret";

#[derive(Serialize, Deserialize)]
struct UserId(String);

struct User {
    pub id: UserId,
}

impl AuthData<UserId> for User {
    fn get_session_data(&self) -> &UserId {
        &self.id
    }

    fn is_totp_active(&self) -> bool {
        true
    }

    fn get_totp_secret(&self) -> &str {
        USER_TOTP_SECRET
    }
}

struct Connection;

impl FromRequest for Connection {
    type Error = Error;
    type Future = Result<Self, Self::Error>;
    type Config = ();

    fn from_request(_req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        Ok(Connection)
    }
}

impl AuthService<UserId> for User {
    type Context = Connection;
    type AuthData = User;
    type Error = Error;
    type Future = Result<Option<Self::AuthData>, Self::Error>;

    fn authenticate(username: &str, password: &str, _: &Self::Context) -> Self::Future {
        if username == "test_user" && password == "test_pass" {
            Ok(Some(User {
                id: UserId("test_user_id".to_string()),
            }))
        } else {
            Ok(None)
        }
    }

    fn get_auth_data(_: &UserId, _: &Self::Context) -> Self::Future {
        Ok(Some(User {
            id: UserId("test_user_id".to_string()),
        }))
    }
}

#[test]
fn successful_auth_test() {
    let mut app = init_service(
        App::new()
            .wrap(
                CookieSession::signed(&[0; 32])
                    .name("session")
                    .secure(false),
            )
            .configure(
                AuthConfig::<User, UserId>::new()
                    .path("auth")
                    .configure(),
            ),
    );

    let payload = r#"{"username":"test_user","password":"test_pass","type":"user"}"#;
    let req = TestRequest::post()
        .uri("/auth")
        .set(ContentType::json())
        .set_payload(payload)
        .to_request();
    let res = block_on(app.call(req)).unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let cookie = res.response().cookies().find(|c| c.name() == "session");
    assert!(cookie.is_some());

    let code = TOTP::new(USER_TOTP_SECRET).generate(
        30,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() - 120,
    );
    let payload = format!(r#"{{"code":"{}","type":"totp"}}"#, code);
    let req = TestRequest::post()
        .uri("/auth")
        .cookie(cookie.unwrap())
        .set(ContentType::json())
        .set_payload(payload)
        .to_request();
    let res = block_on(app.call(req)).unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let cookie = res.response().cookies().find(|c| c.name() == "session");
    assert!(cookie.is_some());

    let code = TOTP::new(USER_TOTP_SECRET).generate(
        30,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );
    let payload = format!(r#"{{"code":"{}","type":"totp"}}"#, code);
    let req = TestRequest::post()
        .uri("/auth")
        .cookie(cookie.unwrap())
        .set(ContentType::json())
        .set_payload(payload)
        .to_request();
    let res = block_on(app.call(req)).unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
