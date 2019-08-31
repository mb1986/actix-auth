use actix_web::FromRequest;

pub trait AccountService {
    type Conn: FromRequest;

    fn can_authenticate(username: &str, password: &str, conn: &Self::Conn) -> bool;
}
