use actix_multipart::Multipart;
use actix_web::error::InternalError;
use actix_web::{get, Responder, HttpResponse, http::header::ContentType, post};
use actix_web_flash_messages::{IncomingFlashMessages, FlashMessage};
use actix_web_grants::proc_macro::has_permissions;
use sailfish::TemplateOnce;
use anyhow::Context;

use crate::utils::{e500, error_chain_fmt, see_other};
use crate::filesystem::{FileSystemError, process_multipart_fields, UploadPayload, NonfilePayload, FilePayload};


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
    let field_map = process_multipart_fields(payload)
        .await
        .context("Multipart processing error")
        .map_err(|e| InternalError::from_response(e.into(), see_other("/web/uploads")))?;

    let text_field = field_map.get("text-field");
    match text_field {
        Some(text_field) => {
            match text_field {
                UploadPayload::Nonfile(f) => {
                    let msg = format!("name: text-field | text: {}", f.text);
                    FlashMessage::error(msg).send();
                }
                UploadPayload::File(_) => { }
            }
        }
        None => {
            FlashMessage::error("text-file field not processed").send();
         }
    }

    let file_field = field_map.get("file-field");
    match file_field {
        Some(file_field) => {
            match file_field {
                UploadPayload::Nonfile(_) => { }
                UploadPayload::File(f) => {
                    let msg = format!("name: file-field | filename: {} | tmp_path: {}", f.filename, f.tmp_path.to_string_lossy());
                    FlashMessage::error(msg).send();
                 }
            }
        }
        None => {
            FlashMessage::error("field-file field not processed").send();
         }
    }  
    
    Ok(see_other("/web/uploads"))   
}