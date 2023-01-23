use actix_web::{get, Responder};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

#[tracing::instrument( name = "Home", skip_all)]
#[get("/")]
pub async fn home(flash_messages: IncomingFlashMessages) -> impl Responder {
    let mut home_text= "Home\n".to_string();
    for m in flash_messages.iter() {
        writeln!(home_text, "\t{}", m.content()).unwrap();
    }

    home_text
}