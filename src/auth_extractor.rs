use actix_session::Session;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{dev, Error, FromRequest, HttpRequest, Result};
use serde::{de::DeserializeOwned, Serialize};

use crate::auth_session::AuthSession;

pub struct Auth<T: Serialize + DeserializeOwned> {
    pub user_id: T,
}

impl<T: Serialize + DeserializeOwned> FromRequest for Auth<T> {
    type Error = Error;
    type Future = Result<Self, Self::Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        let maybe_session = Session::from_request(req, payload);
        if let Ok(session) = maybe_session {
            if let Ok(Some(user_id)) = session.get_user_id() {
                AuthSession::<T>::refresh_ttl(&session)
                    .and_then(|()| Ok(Auth { user_id }))
                    .map_err(ErrorInternalServerError)
            } else {
                Err(ErrorUnauthorized("unauthorized"))
            }
        } else {
            Err(ErrorInternalServerError("auth"))
        }
    }
}
