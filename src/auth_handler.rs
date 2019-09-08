use futures::future::{ok, Future};
use futures::IntoFuture;

use actix_session::Session;
use actix_web::{error::ErrorInternalServerError, web, web::Json, Error, HttpResponse, Responder};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "totp")]
use chrono::Utc;
#[cfg(feature = "totp")]
use otpauth::TOTP;

use crate::auth_extractor::Auth;
use crate::auth_service::{AuthData, AuthService};
use crate::auth_session::AuthSession;
use crate::model::AuthRequest;

fn handle_sign_in<T, U>(
    req: Json<AuthRequest>,
    session: Session,
    ctx: T::Context,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>>
where
    T: AuthService<U>,
    U: DeserializeOwned + Serialize,
{
    match &*req {
        AuthRequest::Credentials(ref credentials) => Box::new(
            T::authenticate(&credentials.username, &credentials.password, &ctx)
                .into_future()
                .map_err(T::Error::into)
                .and_then(move |maybe_auth_data| {
                    match &maybe_auth_data {
                        Some(ref auth_data) => session
                            .authenticate(auth_data.get_session_data())
                            .map(|_| {
                                #[cfg(feature = "totp")]
                                return HttpResponse::Unauthorized().finish();

                                #[cfg(not(feature = "totp"))]
                                return HttpResponse::NoContent().finish();
                            })
                            .map_err(|err| ErrorInternalServerError(err)),
                        None => Ok(HttpResponse::Unauthorized().finish()),
                    }
                    .into_future()
                }),
        ),
        #[allow(unused_variables)]
        AuthRequest::TotpCode(ref totp_code) => {
            #[cfg(feature = "totp")]
            {
                // TODO: check session...
                // T::get_totp_secret
                let secret = "secret";
                let totp = TOTP::new(secret);
                return Box::new(ok(
                    if totp.verify(
                        totp_code.code.parse().unwrap_or(0),
                        30,
                        Utc::now().timestamp() as u64,
                    ) {
                        HttpResponse::NoContent().finish()
                    } else {
                        HttpResponse::Unauthorized().finish()
                    },
                ));
            }
            #[cfg(not(feature = "totp"))]
            {
                return Box::new(ok(HttpResponse::BadRequest().finish()));
            }
        }
    }
}

fn handle_sign_out<T, U>(_: Auth<U>, session: Session) -> impl Responder
where
    T: AuthService<U>,
    U: DeserializeOwned + Serialize,
{
    session.purge();
    HttpResponse::NoContent().finish()
}

pub fn auth_handler_config<T, U>(cfg: &mut web::ServiceConfig)
where
    T: AuthService<U> + 'static,
    U: DeserializeOwned + Serialize + 'static,
{
    cfg.service(
        web::scope("/auth").service(
            web::resource("")
                .route(web::post().to_async(handle_sign_in::<T, U>))
                .route(web::delete().to(handle_sign_out::<T, U>)),
        ),
    );
}
