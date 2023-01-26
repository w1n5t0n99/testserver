use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, Responder, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;
use sea_orm::DbConn;
use actix_web_flash_messages::FlashMessage;

use crate::db::*;
use crate::auth::Client;
use crate::utils::{error_chain_fmt, see_other};


#[tracing::instrument( name = "assets", skip_all)]
#[has_permissions("admin")]
#[get("/assets")]
pub async fn assets(client: web::ReqData<Client>, db: web::Data<DbConn>) -> impl Responder {
    let assets = find_all_assets(&db).await;
    let client = client.into_inner();

    let mut body = "".to_string();
    body.push_str(format!("<p><label>USER ID {}: CURRENTLY LOGGED IN</label>\n", client.user_id).as_str());
    for r in client.roles {
        body.push_str(format!("<label>USER ROLE - {}</label>\n", r).as_str());
    }
    body.push_str("</p>");

    body.push_str("<p>");
    if let Ok(assets) = assets {
        for asset in &assets {
            let line = format!("<label>{} - {}</label>\n", asset.name, asset.description);
            body.push_str(line.as_str());
        }
    }
    else {
        body = "no assets found!".to_string();
    }
    body.push_str("</p>\n");

    let html_body = format!(
        r#"<!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Asset</title>
            </head>
            <body>
                <label>Asset Page!</label>
                {body}
                <form action="/web/assets/add" method="post">  
                    <label>Test Insert Asset form</label>  
                    <label>Name
                        <input 
                            type="text" 
                            placeholder="Enter Asset Name" 
                            name="name"
                        >
                    </label>
                    <label>Description
                        <input 
                            type="text" 
                            placeholder="Enter Description"
                            name="description"
                        >
                    </label>   
                    <button type="submit">Add</button>
                </form>
            </body>
        </html>"#
    );

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_body) 
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