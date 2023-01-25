use actix_web::{get, Responder, HttpResponse, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

#[tracing::instrument( name = "Home", skip_all)]
#[get("/home")]
pub async fn home(flash_messages: IncomingFlashMessages) -> impl Responder {
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
                <label>Home Page!</label>
                {error_html}
                <form action="/web/logout" method="post">  
                    <label>Test logout form</label>     
                    <button type="submit">Logout</button>
                </form>
            </body>
        </html>"#
    );

   
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body) 
    
}