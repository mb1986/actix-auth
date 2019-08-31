use std::marker::PhantomData;

use actix_web::web::ServiceConfig;

use crate::auth_handler::auth_handler_config;
use crate::user_account::AccountService;

pub struct AuthConfig<T: AccountService + 'static> {
    path: &'static str,
    session_ttl: i64,
    user: PhantomData<T>,
}

impl<T: AccountService> AuthConfig<T> {
    pub fn new() -> Self {
        AuthConfig {
            path: "/auth",
            session_ttl: 86400,
            user: PhantomData,
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
            auth_handler_config::<T>(cfg);
        }
    }
}
