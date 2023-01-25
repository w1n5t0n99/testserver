use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};
use crate::db::*;

use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::web::{Data, ReqData};
use actix_web::{FromRequest, HttpMessage};
use actix_web_lab::middleware::Next;
use sea_orm::DbConn;
use std::ops::Deref;
use uuid::Uuid;


#[derive(Copy, Clone, Debug)]
pub struct UserId(Uuid);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for UserId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await
    }?;

    match session.get_user_id().map_err(e500)? {
        Some(user_id) => {
            req.extensions_mut().insert(UserId(user_id));
            next.call(req).await
        }
        None => {
            Err(InternalError::from_response(
                anyhow::anyhow!("The user has not logged in"),
             see_other("/login")).into())
        }
    }
}

pub async fn extract_user_roles(req: &mut ServiceRequest) -> Result<Vec<String>, actix_web::Error> {
    let user_id = req
        .extract::<ReqData<UserId>>()
        .await
        .map_err(|_e| e500("User ID not found")
        )?;

    let db_conn = req
        .app_data::<Data<DbConn>>()
        .ok_or_else(|| e500("Database connection extractor not found"))?;

    let user_id = user_id.into_inner();

    let roles = find_user_roles(*user_id, db_conn)
        .await
        .map_err(|e| e500(e))?;

    Ok(roles)    
}