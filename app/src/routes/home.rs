use actix_web::{get, Responder};

#[tracing::instrument( name = "Home", skip_all)]
#[get("/")]
pub async fn home() -> impl Responder {
    "Hello world!"
}