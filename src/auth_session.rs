use actix_session::Session;
use actix_web::{error::ErrorUnauthorized, Result};
use serde::{de::DeserializeOwned, Serialize};

const SESSION_USER_ID: &str = "user_id";

pub trait AuthSession<T: Serialize + DeserializeOwned> {
    fn authenticate(&self, user_id: &T) -> Result<()>;
    fn get_user_id(&self) -> Result<Option<T>>;
    fn refresh_ttl(&self) -> Result<()>;
}

impl<T: Serialize + DeserializeOwned> AuthSession<T> for Session {
    fn authenticate(&self, user_id: &T) -> Result<()> {
        self.set(SESSION_USER_ID, user_id)?;
        self.renew();
        Ok(())
    }

    fn get_user_id(&self) -> Result<Option<T>> {
        self.get::<T>(SESSION_USER_ID)
    }

    fn refresh_ttl(&self) -> Result<()> {
        self.get::<T>(SESSION_USER_ID)
            .and_then(|ref maybe_user_id| match maybe_user_id {
                Some(ref user_id) => self.set(SESSION_USER_ID, user_id),
                None => Err(ErrorUnauthorized("can not refresh ttl")),
            })
    }
}
