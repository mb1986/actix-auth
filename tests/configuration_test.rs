use actix_http::HttpService;
use actix_http_test::TestServer;
use actix_web::App;

use actix_auth::{AccountService, AuthConfig};

#[test]
fn configuration_test() {
    #[derive(Clone)]
    struct UserService;

    impl UserService {
        pub fn new() -> Self {
            UserService
        }
    }

    impl AccountService for UserService {
        fn find_user(&self, _username: &str, _password: &str) -> &'static str {
            "test"
        }
    }

    let mut srv = TestServer::new(|| {
        let us = UserService::new();
        HttpService::new(
            App::new().configure(
                AuthConfig::new(us)
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
