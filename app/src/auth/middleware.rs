use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};
use crate::auth::{Client, ClientError};

use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::web::{Data, ReqData};
use actix_web::{FromRequest, HttpMessage};
use actix_web_lab::middleware::Next;
use sea_orm::DbConn;


pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let session = {
        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await
    }?;

    let db = req
        .app_data::<Data<DbConn>>()
        .ok_or_else(|| e500("Database connection extractor not found"))?;

    let client = Client::from_user_session(&session, db).await;
   
    match client {
        Ok(client) => {
            req.extensions_mut().insert(client.clone());
            next.call(req).await
        }
        Err(ClientError::MissingUserSession) => {
            Err(InternalError::from_response(
                anyhow::anyhow!("The user has not logged in"),
                see_other("/login")).into())
        }
        Err(ClientError::UnexpectedError(e)) => {
            Err(e500(e))
        }
    }
}

pub async fn extract_user_roles(req: &mut ServiceRequest) -> Result<Vec<String>, actix_web::Error> {
    let client = req
        .extract::<ReqData<Client>>()
        .await
        .map_err(|_e| e500("User client not found")
        )?;

    Ok(client.roles.clone())    
}