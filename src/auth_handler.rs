use futures::future::{ok, Future};
use futures::IntoFuture;

use actix_session::Session;
use actix_web::{error::ErrorInternalServerError, web, web::Json, Error, HttpResponse, Responder};

use crate::auth_service::AuthService;
use crate::model::AuthRequest;

use crate::auth_session::AuthSession;

fn sign_in<T: AuthService>(
    req: Json<AuthRequest>,
    session: Session,
    conn: T::Conn,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    match &*req {
        AuthRequest::Credentials(ref credentials) => Box::new(
            T::can_authenticate(&credentials.username, &credentials.password, &conn)
                .into_future()
                .map_err(|err| err.into())
                .and_then(move |can| {
                    if can {
                        session
                            .authenticate("mb")
                            .map(|_| HttpResponse::NoContent().finish())
                            .map_err(|err| ErrorInternalServerError(err))
                    } else {
                        Ok(HttpResponse::Unauthorized().finish())
                    }
                    .into_future()
                }),
        ),
        AuthRequest::TotpCode(totp) => Box::new(ok(HttpResponse::NoContent().finish())),
    }
}

fn sign_out(/*_: Auth, */ session: Session) -> impl Responder {
    session.purge();
    HttpResponse::NoContent().finish()
}

// fn info(auth: Auth) -> impl Responder {
//     HttpResponse::Ok().body(auth.user_id)
// }

pub fn auth_handler_config<T>(cfg: &mut web::ServiceConfig)
where
    T: AuthService + 'static,
{
    cfg.service(
        web::scope("/auth").service(
            web::resource("")
                .route(web::post().to_async(sign_in::<T>))
                .route(web::delete().to(sign_out)), // .route(web::get().to(info)),
        ),
    );
}
