use actix_web::dev::{Payload, Service};
use actix_web::http::{header::ContentType, StatusCode};
use actix_web::test::{block_on, init_service, TestRequest};
use actix_web::{App, Error, FromRequest, HttpRequest};

use actix_auth::{AuthConfig, AuthService};

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
    type Conn = Connection;
    type Error = Error;
    type Future = Result<bool, Self::Error>;

    fn can_authenticate(_username: &str, _password: &str, _: &Self::Conn) -> Self::Future {
        Err(Error::from(()))
    }
}

#[test]
fn internal_server_error_test() {
    let mut app = init_service(
        App::new().configure(
            AuthConfig::<User>::new()
                .path("auth")
                .session_ttl(259200)
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
    assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
