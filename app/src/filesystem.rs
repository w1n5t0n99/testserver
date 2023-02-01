use std::{path::PathBuf, collections::HashMap};
use actix_multipart::{Multipart, Field};
use actix_web::http::header::DispositionType;
use blake3::Hash;
use futures::{TryStreamExt, StreamExt};
use mime::Mime;
use anyhow::{anyhow, bail, Context};

use crate::utils::error_chain_fmt;



#[derive(thiserror::Error)]
pub enum FileSystemError {
    #[error("Error processing payload")]
    Payload(#[source] anyhow::Error),
    #[error("Something went wrong")]
    Unexpected(#[from] anyhow::Error),
}

impl std::fmt::Debug for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Clone)]
pub enum UploadPayload {
    File(FilePayload),
    Nonfile(NonfilePayload),
}

#[derive(Clone)]
pub struct FilePayload {
    pub data: Vec<u8>,
    pub filename: String,
    pub hash: blake3::Hash,
    pub tmp_path: PathBuf,
    pub mime: Mime,
}

#[derive(Clone)]
pub struct NonfilePayload {
    pub data: Vec<u8>,
    pub text: String,
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

pub async fn process_multipart_fields(mut mutipart: Multipart) -> Result<HashMap<String, UploadPayload>, FileSystemError> {
    let mut payload_map = HashMap::new();

    while let Ok(Some(mut field)) = mutipart.try_next().await {
        let name = field.content_disposition()
            .get_name()
            .context("Field name not found")
            .map_err(|e| FileSystemError::Payload(anyhow!(e)))?
            .to_string();

        let mime = field.content_type();
        match mime {
            None => {
                let f = process_nonfile_field(&mut field).await?;
                match f {
                    Some(f) => { payload_map.insert(name, UploadPayload::Nonfile(f)); }
                    None => { }
                }
            }
            Some(_) => {
                let f = process_file_field(&mut field).await?;
                match f {
                    Some(f) => { payload_map.insert(name, UploadPayload::File(f)); }
                    None => { }
                }
            }
        }
    }

    Ok(payload_map)
}

async fn process_nonfile_field(field: &mut Field) -> Result<Option<NonfilePayload>, FileSystemError> {

    let mut data: Vec<u8> = Vec::with_capacity(1024);
    while let Some(chunk) = field.next().await {
        let byte = chunk.map_err(|e| FileSystemError::Payload(anyhow!(e)))?;
        data.extend(byte.to_owned());
    }

    if data.is_empty() {
        return Ok(None);
    }

    let text = match std::str::from_utf8(&data) {
        Ok(text) => text.to_string(),
        Err(e) => { return Ok(None); }
    };

    Ok(Some(NonfilePayload { data, text } ))
}

async fn process_file_field( field: &mut Field) -> Result<Option<FilePayload>, FileSystemError> {

    let filename = field
        .content_disposition()
        .get_filename()
        .context("field missing filename")
        .map_err(|e| FileSystemError::Payload(e))?
        .to_string();

    let mime = field
        .content_type()
        .context("field mime not found")
        .map_err(|e| FileSystemError::Payload(e))?
        .clone();

    let ext = new_mime_guess::get_mime_extensions_str(mime.type_().as_str())
        .context("field ext not found")
        .map_err(|e| FileSystemError::Payload(e))?;

    let tmp_path = format!("./tempfiles/{}.{}", uuid::Uuid::new_v4(), ext[0]);

    let mut hasher = blake3::Hasher::new();
    let mut data: Vec<u8> = Vec::with_capacity(1024);
    while let Some(chunk) = field.next().await {
        let bytes = chunk.map_err(|e| FileSystemError::Payload(anyhow!(e)))?;

        hasher.update(&bytes);
        data.extend(bytes.to_owned());
    }

    if data.is_empty() {
        return Ok(None);
    }

    Ok(Some(FilePayload { 
            data: data,
            filename: filename.to_string(),
            hash: hasher.finalize(),
            tmp_path: tmp_path.into(),
            mime: mime 
        } 
    ))
}
