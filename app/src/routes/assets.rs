use actix_web::{get, web, Responder};
use sea_orm::DbConn;
use secrecy::Secret;
use tracing_subscriber::fmt::format;

use crate::auth::{validate_credentials, Credentials, AuthError};
use crate::session_state::TypedSession;
use crate::db::*;


#[tracing::instrument( name = "view assets", skip_all)]
#[get("/assets")]
pub async fn assets(db: web::Data<DbConn>) -> impl Responder {
    let assets = find_all_assets(&db).await;

    let mut body = "".to_string();
    if let Ok(assets) = assets {
        for asset in &assets {
            let line = format!("{} - {}\n", asset.name, asset.description);
            body.push_str(line.as_str());
        }
    }
    else {
        body = "no assets found!".to_string();
    }

    body
}