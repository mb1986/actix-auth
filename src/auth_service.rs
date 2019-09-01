use actix_web::{Error, FromRequest};
use futures::IntoFuture;
use serde::{de::DeserializeOwned, Serialize};

pub trait AuthService: Sized {
    type Context: FromRequest;
    type UserData: DeserializeOwned + Serialize;
    type Error: Into<Error> + 'static;
    type Future: IntoFuture<Item = Option<Self::UserData>, Error = Self::Error> + 'static;

    fn authenticate(username: &str, password: &str, ctx: &Self::Context) -> Self::Future;
    // fn find_user_id
    // verify_user
    // authenticate
    // fn get_totp_secret
}
