use std::marker::PhantomData;

use actix_web::web::ServiceConfig;
use serde::{de::DeserializeOwned, Serialize};

use crate::auth_handler::auth_handler_config;
use crate::auth_service::AuthService;

pub struct AuthConfig<T: AuthService<U> + 'static, U: DeserializeOwned + Serialize + 'static> {
    path: &'static str,
    session_ttl: i64,
    auth: PhantomData<T>,
    data: PhantomData<U>,
}

impl<T: AuthService<U>, U: DeserializeOwned + Serialize> AuthConfig<T, U> {
    pub fn new() -> Self {
        AuthConfig {
            path: "/auth",
            session_ttl: 86400,
            auth: PhantomData,
            data: PhantomData,
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
            auth_handler_config::<T, U>(cfg);
        }
    }
}
