use actix_session::Session;
use actix_web::{web, HttpResponse, Responder, web::Json};

use crate::user_account::AccountService;
use crate::model::AuthRequest;

// use super::{Auth, AuthSession};

fn sign_in<T>(req: Json<AuthRequest>, _session: Session, conn: T::Conn) -> impl Responder where T: AccountService {
    // let u = T::find_user(&s, "username", "password");
    let result = T::can_authenticate("", "", &conn);
    println!("Can authenticate: {:?}", result);
    println!("Req: {:?}", *req);
    // session
    //     .authenticate("mb")
    //     .map(|_| HttpResponse::NoContent().finish())
    //     .map_err(|_| HttpResponse::InternalServerError().finish())
    HttpResponse::Ok().finish()
}

// fn sign_out(_: Auth, session: Session) -> impl Responder {
//     session.purge();
//     HttpResponse::NoContent().finish()
// }

// fn info(auth: Auth) -> impl Responder {
//     HttpResponse::Ok().body(auth.user_id)
// }
pub fn auth_handler_config<T>(cfg: &mut web::ServiceConfig) where T: AccountService + 'static {
    cfg.service(
        web::scope("/auth").service(
            web::resource("")
                .route(web::post().to(sign_in::<T>))
                // .route(web::delete().to(sign_out))
                // .route(web::get().to(info)),
        ),
    );
}
