use actix_session::Session;
use actix_web::{error::ErrorUnauthorized, Result};
use serde::{de::DeserializeOwned, Serialize};

const SESSION_USER_DATA: &str = "user";

pub trait AuthSession<T: Serialize + DeserializeOwned> {
    fn authenticate(&self, user_data: &T) -> Result<()>;
    fn get_user_data(&self) -> Result<Option<T>>;
    fn refresh_ttl(&self) -> Result<()>;
}

impl<T: Serialize + DeserializeOwned> AuthSession<T> for Session {
    fn authenticate(&self, user_data: &T) -> Result<()> {
        self.set(SESSION_USER_DATA, user_data)?;
        self.renew();
        Ok(())
    }

    fn get_user_data(&self) -> Result<Option<T>> {
        self.get::<T>(SESSION_USER_DATA)
    }

    fn refresh_ttl(&self) -> Result<()> {
        self.get::<T>(SESSION_USER_DATA)
            .and_then(|ref maybe_user_data| match maybe_user_data {
                Some(ref user_data) => self.set(SESSION_USER_DATA, user_data),
                None => Err(ErrorUnauthorized("can not refresh ttl")),
            })
    }
}
