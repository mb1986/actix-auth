use actix_http::HttpService;
use actix_http_test::TestServer;
use actix_web::{dev, App, Error, FromRequest, HttpRequest};

use actix_auth::{AccountServiceEx, AuthConfig};

#[test]
fn configuration_test() {
    // #[derive(Clone)]
    // struct UserService;

    // impl UserService {
    //     pub fn new() -> Self {
    //         UserService
    //     }
    // }

    // impl AccountService for UserService {
    //     fn find_user(&self, _username: &str, _password: &str) -> &'static str {
    //         "test"
    //     }
    // }

    struct User {}

    struct Connection {}

    impl FromRequest for Connection {
        type Error = Error;
        type Future = Result<Self, Self::Error>;
        type Config = ();

        fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
            Ok(Connection {})
        }
    }

    impl AccountServiceEx for User {
        type Conn = Connection;
        fn find_user(_username: &str, _password: &str, _conn: &Connection) -> String {
            "test 2".to_string()
        }
    }

    let mut srv = TestServer::new(|| {
        HttpService::new(
            App::new().configure(
                AuthConfig::<User>::new()
                    .path("auth")
                    .session_ttl(259200)
                    .configure(),
            ),
        )
    });

    let req = srv.post("/auth");
    let response = srv.block_on(req.send()).unwrap();
    assert!(response.status().is_success());
}
