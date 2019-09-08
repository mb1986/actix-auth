use actix_web::{Error, FromRequest};
use futures::IntoFuture;
use serde::{de::DeserializeOwned, Serialize};


pub trait AuthData<T: DeserializeOwned + Serialize> {
    fn get_session_data(&self) -> &T;

    #[cfg(feature = "totp")]
    fn is_totp_active(&self) -> bool;

    #[cfg(feature = "totp")]
    fn get_totp_secret(&self) -> &str;
}

pub trait AuthService<T: DeserializeOwned + Serialize>: Sized {
    type Context: FromRequest + 'static;
    type AuthData: AuthData<T>;
    type Error: Into<Error> + 'static;
    type Future: IntoFuture<Item = Option<Self::AuthData>, Error = Self::Error> + 'static;

    fn authenticate(username: &str, password: &str, ctx: &Self::Context) -> Self::Future;

    #[cfg(feature = "totp")]
    fn get_auth_data(session_data: &T, ctx: &Self::Context) -> Self::Future;
}
