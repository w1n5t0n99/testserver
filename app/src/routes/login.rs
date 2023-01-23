use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{get, post, Responder, web, HttpResponse};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use sea_orm::DbConn;
use secrecy::Secret;

use crate::auth::{validate_credentials, Credentials, AuthError};
use crate::session_state::TypedSession;
use crate::utils::{see_other, error_chain_fmt};
use std::fmt::Write;


#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(e.to_string()).send();
    InternalError::from_response(e, see_other("/login"))
}

#[get("/login")]
pub async fn view_login(flash_messages: IncomingFlashMessages) -> impl Responder {
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    let body = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Login</title>
            </head>
            <body>
                {error_html}
                <form action="/login" method="post">  
                    <label>Test login form</label>     
                    <button type="submit">Login</button>
                </form>
            </body>
        </html>"#
    );

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body)        
}

#[tracing::instrument(
    name = "login",
    skip_all,
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
#[post("/login")]
pub async fn post_login (
    db: web::Data<DbConn>,
    session: TypedSession,
) -> Result<impl Responder, InternalError<LoginError>> {
    // Just login as admin for testing auth
    let credentials = Credentials {
        username: "admin".to_string(),
        password: Secret::new("everythinghastostartsomewhere".to_string())
    };

    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
    match validate_credentials(credentials, &db).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;

            Ok(see_other("/home"))
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            Err(login_redirect(e))
        }
    }
}
