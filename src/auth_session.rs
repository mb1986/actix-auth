use actix_session::Session;
use actix_web::{error::ErrorUnauthorized, Result};

const SESSION_USER_ID: &str = "user_id";

pub trait AuthSession {
    fn authenticate(&self, user_id: &str) -> Result<()>;
    fn get_user_id(&self) -> Result<Option<String>>;
    fn refresh_ttl(&self) -> Result<()>;
}

impl AuthSession for Session {
    fn authenticate(&self, user_id: &str) -> Result<()> {
        self.set(SESSION_USER_ID, user_id)?;
        self.renew();
        Ok(())
    }

    fn get_user_id(&self) -> Result<Option<String>> {
        self.get::<String>(SESSION_USER_ID)
    }

    fn refresh_ttl(&self) -> Result<()> {
        self.get::<String>(SESSION_USER_ID)
            .and_then(|ref maybe_user_id| match maybe_user_id {
                Some(ref user_id) => self.set(SESSION_USER_ID, user_id),
                None => Err(ErrorUnauthorized("can not refresh ttl")),
            })
    }
}
