use std::ops::Deref;

use actix_session::Session;
use actix_web::{error::ErrorUnauthorized, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

const SESSION_AUTH_DATA: &str = "auth";

#[derive(Serialize, Deserialize)]
pub struct AuthSessionData<T> {
    pub auth_data: T,

    #[cfg(feature = "totp")]
    pub totp_verified: bool,
}

impl<T> Deref for AuthSessionData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.auth_data
    }
}

pub trait AuthSession<T: Serialize + DeserializeOwned> {
    fn authenticate(&self, user_data: &T) -> Result<()>;
    #[cfg(feature = "totp")]
    fn totp_verify(&self) -> Result<()>;
    fn get_auth_data(&self) -> Result<Option<AuthSessionData<T>>>;
    fn refresh_ttl(&self) -> Result<()>;
}

impl<T: Serialize + DeserializeOwned> AuthSession<T> for Session {
    fn authenticate(&self, auth_data: &T) -> Result<()> {
        self.set(
            SESSION_AUTH_DATA,
            AuthSessionData {
                auth_data,

                #[cfg(feature = "totp")]
                totp_verified: false,
            },
        )?;
        self.renew();
        Ok(())
    }

    #[cfg(feature = "totp")]
    fn totp_verify(&self) -> Result<()> {
        self.get::<AuthSessionData<T>>(SESSION_AUTH_DATA)
            .and_then(|maybe_auth_data| match maybe_auth_data {
                Some(auth_session_data) => self.set(
                    SESSION_AUTH_DATA,
                    AuthSessionData {
                        auth_data: auth_session_data.auth_data,
                        totp_verified: true,
                    },
                ),
                None => Err(ErrorUnauthorized("can not refresh ttl")),
            })?;
        self.renew();
        Ok(())
    }

    fn get_auth_data(&self) -> Result<Option<AuthSessionData<T>>> {
        self.get::<AuthSessionData<T>>(SESSION_AUTH_DATA)
    }

    fn refresh_ttl(&self) -> Result<()> {
        self.get::<AuthSessionData<T>>(SESSION_AUTH_DATA)
            .and_then(|ref maybe_auth_data| match maybe_auth_data {
                Some(ref auth_data) => self.set(SESSION_AUTH_DATA, auth_data),
                None => Err(ErrorUnauthorized("can not refresh ttl")),
            })
    }
}
