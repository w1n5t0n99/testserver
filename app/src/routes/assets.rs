use actix_web::{get, web, Responder};
use actix_web_grants::proc_macro::has_permissions;
use sea_orm::DbConn;

use crate::db::*;
use crate::auth::UserId;


#[tracing::instrument( name = "assets", skip_all)]
#[has_permissions("admin")]
#[get("/assets")]
pub async fn assets(user_id: web::ReqData<UserId>, db: web::Data<DbConn>) -> impl Responder {
    let assets = find_all_assets(&db).await;
    let user_id = user_id.into_inner();

    let mut body = "".to_string();
    body.push_str(format!("USER ID {}: CURRENTLY LOGGED IN\n\n", *user_id).as_str());
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