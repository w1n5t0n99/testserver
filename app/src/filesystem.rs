use std::path::PathBuf;
use actix_multipart::{Multipart, Field};
use actix_web::http::header::DispositionType;
use futures::TryStreamExt;
use mime::Mime;

use crate::utils::error_chain_fmt;



#[derive(thiserror::Error)]
pub enum FileSystemError {
    #[error("Error parsing multipart form")]
    Multipart(#[source] anyhow::Error),
    #[error("Something went wrong")]
    Unexpected(#[from] anyhow::Error),
}

impl std::fmt::Debug for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

/* 
    Represents a single multipart file field that was
    uploaded and saved to tmp file on server
*/
pub struct UploadPayload {
    pub data: Vec<u8>,
    pub filename: String,
    pub file_hash: blake3::Hash,
    pub tmp_path: PathBuf,
    pub mime: Mime,
}

pub struct FieldMeta {
    pub name: String,
    pub filename: String,
    pub disp_type: String,
    pub content_type: String,
}

pub async fn parse_multipart_form(mut mutipart: Multipart) -> Result<Vec<FieldMeta>, FileSystemError> {
    // see: https://users.rust-lang.org/t/file-upload-in-actix-web/64871/3
    let mut fields_vec = Vec::new();
    // Iterate over multipart stream
    while let Ok(Some(mut field)) = mutipart.try_next().await {
        let cdispostion = field.content_disposition();
        let ctype = match field.content_type() {
            Some(c) => {
                format!("{}//{}", c.type_(), c.subtype())
            }
            None => "none".to_string(),
        };

        let disp_type = match cdispostion.disposition {
            DispositionType::Inline => "inline".to_string(),
            DispositionType::FormData => "form data".to_string(),
            DispositionType::Attachment => "attachment".to_string(),
            DispositionType::Ext(_) => "ext".to_string(),
        };

        
        
        
        let fmeta = FieldMeta {
            name: cdispostion.get_name().unwrap_or("none").to_owned(),
            filename: cdispostion.get_filename().unwrap_or("none").to_owned(),
            disp_type: disp_type,
            content_type: ctype,
        };

        fields_vec.push(fmeta);
    }

    Ok(fields_vec)
}

// Direct way of converting an actix_multipart field into an upload response.
pub async fn insert_field_as_attachment(field: &mut Field) -> Result<(), FileSystemError> {
    // Save the file to a temporary location and get payload data.
    // Pass file through deduplication and receive a response..
    Ok(())
}
