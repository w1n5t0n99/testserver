use actix_web::rt::task::JoinHandle;
use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use validator::ValidationErrors;
use sea_orm::{DbErr, RuntimeErr};

// Return an opaque 500 while preserving the error root's cause for logging.
pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

// Return a 400 with the user-representation of the validation error as body.
// The error root cause is preserved for logging purposes.
pub fn e400<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorBadRequest(e)
}

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    actix_web::rt::task::spawn_blocking(move || current_span.in_scope(f))
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

pub trait ResultValidateExt {
    fn is_field_valid(&self, field: &str) -> bool;
}

impl<T> ResultValidateExt for Result<T, ValidationErrors>
{
    fn is_field_valid(&self, field: &str) -> bool {
        match self {
            Ok(_) => true,
            Err(e) => {
                let emap = e.errors();
                if let Some(_e) = emap.get(field) {
                    return false;
                }

                true
            }
        }
    }
}

pub trait ResultDbExt {
    fn is_duplicate_key(&self) -> bool;
}

// TODO: I think need to include sqlx to access error type
impl<T> ResultDbExt for Result<T, DbErr> {
    fn is_duplicate_key(&self) -> bool {
        match self {
            Ok(_) => true,
            Err(e) => { true }
        }
    }
}
