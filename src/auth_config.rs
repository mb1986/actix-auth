use actix_web::web::ServiceConfig;

use crate::auth_handler::auth_handler_config;
use crate::user_account::AccountService;

pub struct AuthConfig<T> where T: AccountService + 'static {
    path: &'static str,
    session_ttl: i64,
    s: T,
}

impl<T> AuthConfig<T> where T: AccountService {
    pub fn new(s: T) -> Self {
        AuthConfig {
            path: "/auth",
            session_ttl: 86400,
            s: s,
        }
    }

    pub fn path(mut self, path: &'static str) -> Self {
        self.path = path;
        self
    }

    pub fn session_ttl(mut self, ttl: i64) -> Self {
        self.session_ttl = ttl;
        self
    }

    pub fn configure(self) -> impl Fn(&mut ServiceConfig) -> () {
        move |cfg| {
            cfg.data(self.s.clone());
            auth_handler_config::<T>(cfg);
        }
    }
}
