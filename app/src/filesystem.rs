use std::path::PathBuf;
use actix_multipart::{Multipart, Field};
use futures::TryStreamExt;
use mime::Mime;

use crate::utils::error_chain_fmt;



#[derive(thiserror::Error)]
pub enum FileSystemError {
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
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
    data: Vec<u8>,
    filename: String,
    file_hash: blake3::Hash,
    tmp_path: PathBuf,
    mime: Mime,
}

pub async fn put_file(mut mutipart: Multipart) -> Result<(), FileSystemError> {
    // see: https://users.rust-lang.org/t/file-upload-in-actix-web/64871/3

    // Iterate over multipart stream
    while let Ok(Some(mut field)) = mutipart.try_next().await {
        
    }

    Ok(())
}

// Direct way of converting an actix_multipart field into an upload response.
pub async fn insert_field_as_attachment(field: &mut Field) -> Result<(), FileSystemError> {
    // Save the file to a temporary location and get payload data.
    // Pass file through deduplication and receive a response..
    Ok(())
}
