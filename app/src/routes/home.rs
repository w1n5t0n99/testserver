use actix_web::{get, Responder, HttpResponse, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;
use sailfish::TemplateOnce;

use crate::utils::e500;


#[derive(TemplateOnce)]
#[template(path = "home.stpl")]
struct HomePage<'a> {
    pub messages: Vec<&'a str>,
}

#[tracing::instrument( name = "Home", skip_all)]
#[get("/home")]
pub async fn home(flash_messages: IncomingFlashMessages) -> Result<impl Responder, actix_web::Error> {
    let messages: Vec<&str> = flash_messages.iter().map(|f| f.content()).collect();

    let body = HomePage {
        messages,
    }
    .render_once()
    .map_err(e500)?;
   
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
    
}