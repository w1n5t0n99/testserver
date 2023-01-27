use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, Responder, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;
use entity::asset;
use sea_orm::DbConn;
use actix_web_flash_messages::FlashMessage;

use crate::db::*;
use crate::auth::Client;
use crate::utils::{error_chain_fmt, see_other, e500};
use sailfish::TemplateOnce;


#[derive(TemplateOnce)]
#[template(path = "assets.stpl")]
struct AssetsPage<'a> {
    pub assets_table: &'a[asset::Model],
    pub messages: Vec<String>,
}

#[tracing::instrument( name = "assets", skip_all)]
#[has_permissions("admin")]
#[get("/assets")]
pub async fn assets(client: web::ReqData<Client>, db: web::Data<DbConn>) -> Result<impl Responder, actix_web::Error> {
    let assets = find_all_assets(&db)
        .await
        .map_err(e500)?;
        
    let client = client.into_inner();

    let body = AssetsPage {
        assets_table: assets.as_slice(),
        messages: vec!["This is a test flash message".to_string()],
    }
    .render_once()
    .map_err(e500)?;
    
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}

#[derive(thiserror::Error)]
pub enum AddError {
    #[error("Validation failed")]
    Validation(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for AddError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[tracing::instrument( name = "add assets", skip_all)]
#[has_permissions("admin")]
#[post("/assets/add")]
pub async fn add_asset(asset_model: web::Form<entity::asset::Model>, db: web::Data<DbConn>) -> Result<HttpResponse, InternalError<AddError>> { 
    // asset.validate()

    let asset_model = asset_model.into_inner();
    let res = insert_asset(asset_model, &db)
        .await
        .map_err(|e| AddError::UnexpectedError(e.into()));

    match res {
        Ok(_) => {
            FlashMessage::info("Asset successfully added.").send();
            Ok(see_other("/web/assets"))
        }
        Err(e) => {
            FlashMessage::info("Error: Could Not Add Asset.").send();
            Err(InternalError::from_response(e, see_other("/web/assets")))
        }
    }
}