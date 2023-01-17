use actix_web::{get, web, Responder};
use sea_orm::DbConn;

#[tracing::instrument( name = "Assets", skip_all())]
#[get("/assets")]
pub async fn assets(db: web::Data<DbConn>) -> impl Responder {
    "Assets Page"
}