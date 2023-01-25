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


#[derive(Clone, Debug)]
pub struct ClientCtx {
    pub user_id: Uuid,
    pub user_name: String,
    pub roles: Vec<String>,
}



