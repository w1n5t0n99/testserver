use actix_web::http::header::ContentType;
use actix_web::{get, Responder, web, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use sea_orm::DbConn;
use secrecy::Secret;
use tracing_subscriber::fmt::format;

use crate::auth::{validate_credentials, Credentials, AuthError};
use crate::session_state::TypedSession;
use crate::db::*;


#[get("/login")]
pub async fn view_login(
    flash_messages: IncomingFlashMessages,
    db: web::Data<DbConn>,
    session: TypedSession,
) -> impl Responder {
    let body = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Login</title>
            </head>
            <body>
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
