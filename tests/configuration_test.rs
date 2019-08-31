use actix_web::dev::{Payload, Service};
use actix_web::http::header::ContentType;
use actix_web::test::{block_on, init_service, TestRequest};
use actix_web::{App, Error, FromRequest, HttpRequest};

use actix_auth::{AccountService, AuthConfig};

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

impl AccountService for User {
    type Conn = Connection;

    fn can_authenticate(_username: &str, _password: &str, _: &Self::Conn) -> bool {
        true
    }
}

#[test]
fn configuration_test() {
    let mut app = init_service(
        App::new().configure(
            AuthConfig::<User>::new()
                .path("auth")
                .session_ttl(259200)
                .configure(),
        ),
    );

    let payload = r#"{"username":"test_user","password":"test_pass","type":"user"}"#;
    // let payload = r#"{"code":"123456","type":"totp"}"#;
    let req = TestRequest::post()
        .uri("/auth")
        .set(ContentType::json())
        .set_payload(payload)
        .to_request();
    let res = block_on(app.call(req)).unwrap();
    assert!(res.status().is_success());
}
