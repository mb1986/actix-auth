use std::marker::PhantomData;

use actix_web::web::ServiceConfig;
use serde::{de::DeserializeOwned, Serialize};

use crate::auth_handler::auth_handler_config;
use crate::auth_service::AuthService;

#[derive(Clone)]
pub struct AuthConfigInner {
    pub path: &'static str,
    #[cfg(feature = "totp")]
    pub totp_timeout: i64,
}

pub struct AuthConfig<T: AuthService<U> + 'static, U: DeserializeOwned + Serialize + 'static> {
    inner: AuthConfigInner,
    _t: PhantomData<(T, U)>,
}

impl<T: AuthService<U>, U: DeserializeOwned + Serialize> AuthConfig<T, U> {
    pub fn new() -> Self {
        AuthConfig {
            inner: AuthConfigInner {
                path: "/auth",
                #[cfg(feature = "totp")]
                totp_timeout: 120,
            },
            _t: PhantomData,
        }
    }

    pub fn path(mut self, path: &'static str) -> Self {
        self.inner.path = path;
        self
    }

    #[cfg(feature = "totp")]
    pub fn totp_timeout(mut self, timeout: i64) -> Self {
        self.inner.totp_timeout = timeout;
        self
    }

    pub fn configure(self) -> impl Fn(&mut ServiceConfig) -> () {
        move |cfg| {
            auth_handler_config::<T, U>(cfg, self.inner.clone());
        }
    }
}
