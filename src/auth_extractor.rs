use std::ops::Deref;

use actix_session::Session;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{dev, Error, FromRequest, HttpRequest, Result};
use serde::{de::DeserializeOwned, Serialize};

use crate::auth_session::AuthSession;

pub struct Auth<T: Serialize + DeserializeOwned> {
    pub data: T,
}

impl<T: DeserializeOwned + Serialize> Deref for Auth<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Serialize + DeserializeOwned> FromRequest for Auth<T> {
    type Error = Error;
    type Future = Result<Self, Self::Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        let maybe_session = Session::from_request(req, payload);
        if let Ok(session) = maybe_session {
            if let Ok(Some(auth_session_data)) = session.get_auth_data() {
                AuthSession::<T>::refresh_ttl(&session)
                    .and_then(|()| {
                        #[cfg(feature = "totp")]
                        return if auth_session_data.totp_verified {
                            Ok(Auth {
                                data: auth_session_data.auth_data,
                            })
                        } else {
                            Err(ErrorUnauthorized("totp not verified"))
                        };

                        #[cfg(not(feature = "totp"))]
                        return Ok(Auth {
                            data: auth_session_data.auth_data,
                        });
                    })
                    .map_err(ErrorInternalServerError)
            } else {
                Err(ErrorUnauthorized("unauthorized"))
            }
        } else {
            Err(ErrorInternalServerError("auth"))
        }
    }
}
