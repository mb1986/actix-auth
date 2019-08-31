use actix_web::FromRequest;

pub trait AccountService: Clone {
    fn find_user(&self, username: &str, password: &str) -> &str;
}

pub trait AccountServiceEx {
    type Conn: FromRequest;
    fn find_user(username: &str, password: &str, conn: &Self::Conn) -> String;
}

pub trait UserAccount {
    fn user_id(&self) -> &str;
}
