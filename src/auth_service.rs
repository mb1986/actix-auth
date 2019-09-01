use actix_web::{Error, FromRequest};
use futures::IntoFuture;

pub trait AuthService: Sized {
    type Conn: FromRequest;
    type Error: Into<Error> + 'static;
    type Future: IntoFuture<Item = bool, Error = Self::Error> + 'static;

    fn can_authenticate(username: &str, password: &str, conn: &Self::Conn) -> Self::Future;
}
