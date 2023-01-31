use actix_multipart::Multipart;
use actix_web::error::InternalError;
use actix_web::{get, Responder, HttpResponse, http::header::ContentType, post};
use actix_web_flash_messages::{IncomingFlashMessages, FlashMessage};
use actix_web_grants::proc_macro::has_permissions;
use futures::TryFutureExt;
use sailfish::TemplateOnce;
use anyhow::Context;

use crate::utils::{e500, error_chain_fmt, see_other};
use crate::filesystem::{FieldMeta, FileSystemError, parse_multipart_form};


#[derive(TemplateOnce)]
#[template(path = "uploads.stpl")]
struct UploadPage<'a> {
    pub messages: Vec<&'a str>,
}

#[tracing::instrument( name = "Upload", skip_all)]
#[has_permissions("admin")]
#[get("/uploads")]
pub async fn uploads(flash_messages: IncomingFlashMessages) -> Result<impl Responder, actix_web::Error> {
    let messages: Vec<&str> = flash_messages.iter().map(|f| f.content()).collect();

    let body = UploadPage {
        messages,
    }
    .render_once()
    .map_err(e500)?;
   
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
    
}

#[derive(thiserror::Error)]
pub enum UploadError {
    #[error("Validation failed")]
    Validation(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for UploadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[tracing::instrument( name = "New Upload", skip_all)]
#[has_permissions("admin")]
#[post("/uploads/new")]
pub async fn new_upload(mut payload: Multipart) -> Result<HttpResponse, InternalError<UploadError>> {
    let fields_meta = parse_multipart_form(payload)
        .await
        .context("Upload error")
        .map_err(|e| InternalError::from_response(e.into(), see_other("/web/uploads")))?;

    for fm in fields_meta {
        let msg = format!("name: {} | filename: {} | type: {}", fm.name, fm.filename, fm.disp_type);
        FlashMessage::error(msg).send();
    }        
    
    Ok(see_other("/web/uploads"))   
}