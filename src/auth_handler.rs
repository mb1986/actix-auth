use futures::future::Future;
use futures::IntoFuture;

use actix_session::Session;
use actix_web::{
    error::ErrorInternalServerError, web, web::Data, web::Json, Error, HttpResponse, Responder,
};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "totp")]
use chrono::Utc;
#[cfg(feature = "totp")]
use otpauth::TOTP;

use crate::auth_config::AuthConfigInner;
use crate::auth_extractor::Auth;
use crate::auth_service::{AuthData, AuthService};
use crate::auth_session::AuthSession;
use crate::model::AuthRequest;

fn handle_sign_in<T, U>(
    req: Json<AuthRequest>,
    session: Session,
    ctx: T::Context,
    config: Data<AuthConfigInner>,
) -> Box<dyn Future<Item = HttpResponse, Error = Error>>
where
    T: AuthService<U>,
    U: DeserializeOwned + Serialize + 'static,
{
    match &*req {
        AuthRequest::Credentials(ref credentials) => Box::new(
            T::authenticate(&credentials.username, &credentials.password, &ctx)
                .into_future()
                .map_err(T::Error::into)
                .and_then(move |maybe_auth_data| match &maybe_auth_data {
                    Some(ref auth_data) => auth_with_credentials(auth_data, &session),
                    None => Err(HttpResponse::Unauthorized().finish().into()),
                }),
        ),
        #[cfg(feature = "totp")]
        AuthRequest::TotpCode(ref totp_code) => {
            let code = totp_code.code.clone();
            Box::new(
                AuthSession::<U>::get_auth_data(&session)
                    .into_future()
                    .and_then(move |maybe_auth_session| match maybe_auth_session {
                        Some(auth_session) => Ok(auth_session),
                        None => Err(HttpResponse::Unauthorized().finish().into()),
                    })
                    .and_then(move |auth_session| {
                        T::get_auth_data(&auth_session.auth_data, &ctx)
                            .into_future()
                            .map_err(T::Error::into)
                            .and_then(move |maybe_auth_data| match &maybe_auth_data {
                                Some(ref auth_data) => auth_with_totp(auth_data, &code, &session),
                                None => Err(HttpResponse::Unauthorized().finish().into()),
                            })
                    }),
            )
        }
    }
}

fn auth_with_credentials<U>(
    auth_data: &impl AuthData<U>,
    session: &Session,
) -> Result<HttpResponse, Error>
where
    U: DeserializeOwned + Serialize,
{
    session
        .authenticate(auth_data.get_session_data())
        .map(|()| {
            #[cfg(feature = "totp")]
            return if auth_data.is_totp_active() {
                HttpResponse::Unauthorized().finish()
            } else {
                HttpResponse::NoContent().finish()
            };

            #[cfg(not(feature = "totp"))]
            return HttpResponse::NoContent().finish();
        })
        .map_err(|err| ErrorInternalServerError(err))
}

#[cfg(feature = "totp")]
fn auth_with_totp<U>(
    auth_data: &impl AuthData<U>,
    totp_code: &str,
    session: &Session,
) -> Result<HttpResponse, Error>
where
    U: DeserializeOwned + Serialize,
{
    let secret = auth_data.get_totp_secret();
    let totp = TOTP::new(secret);
    Ok(
        if totp.verify(
            totp_code.parse().unwrap_or(0),
            30,
            Utc::now().timestamp() as u64,
        ) {
            AuthSession::<U>::totp_verify(session)?;
            HttpResponse::NoContent().finish()
        } else {
            session.purge();
            HttpResponse::Unauthorized().finish()
        },
    )
}

fn handle_sign_out<T, U>(_: Auth<U>, session: Session) -> impl Responder
where
    T: AuthService<U>,
    U: DeserializeOwned + Serialize,
{
    session.purge();
    HttpResponse::NoContent().finish()
}

pub fn auth_handler_config<T, U>(cfg: &mut web::ServiceConfig, auth_config: AuthConfigInner)
where
    T: AuthService<U> + 'static,
    U: DeserializeOwned + Serialize + 'static,
{
    cfg.service(
        web::scope(auth_config.path).data(auth_config).service(
            web::resource("")
                .route(web::post().to_async(handle_sign_in::<T, U>))
                .route(web::delete().to(handle_sign_out::<T, U>)),
        ),
    );
}
