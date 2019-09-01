use actix_web::{Error, FromRequest};
use futures::IntoFuture;
use serde::{de::DeserializeOwned, Serialize};

pub trait AuthService: Sized {
    type Conn: FromRequest;
    type UserId: DeserializeOwned + Serialize;
    type Error: Into<Error> + 'static;
    type Future: IntoFuture<Item = Option<Self::UserId>, Error = Self::Error> + 'static;

    fn authenticate(username: &str, password: &str, conn: &Self::Conn) -> Self::Future;
    // fn find_user_id
    // fn get_totp_secret
}
