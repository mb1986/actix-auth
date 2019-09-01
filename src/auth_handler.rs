use futures::future::{ok, Future};
use futures::IntoFuture;

use actix_session::Session;
use actix_web::{error::ErrorInternalServerError, web, web::Json, Error, HttpResponse, Responder};

use crate::auth_extractor::Auth;
use crate::auth_service::AuthService;
use crate::auth_session::AuthSession;
use crate::model::AuthRequest;

fn handle_sign_in<T: AuthService>(
    req: Json<AuthRequest>,
    session: Session,
    ctx: T::Context,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    match &*req {
        AuthRequest::Credentials(ref credentials) => Box::new(
            T::authenticate(&credentials.username, &credentials.password, &ctx)
                .into_future()
                .map_err(T::Error::into)
                .and_then(move |maybe_user_id| {
                    match &maybe_user_id {
                        Some(ref user_id) => session
                            .authenticate(user_id)
                            .map(|_| HttpResponse::NoContent().finish())
                            .map_err(|err| ErrorInternalServerError(err)),
                        None => Ok(HttpResponse::Unauthorized().finish()),
                    }
                    .into_future()
                }),
        ),
        AuthRequest::TotpCode(totp) => Box::new(ok(HttpResponse::NoContent().finish())),
    }
}

fn handle_sign_out<T: AuthService>(_: Auth<T::UserData>, session: Session) -> impl Responder {
    session.purge();
    HttpResponse::NoContent().finish()
}

pub fn auth_handler_config<T: AuthService + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth").service(
            web::resource("")
                .route(web::post().to_async(handle_sign_in::<T>))
                .route(web::delete().to(handle_sign_out::<T>)),
        ),
    );
}
