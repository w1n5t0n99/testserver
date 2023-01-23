use actix_web::{get, web, Responder};
use sea_orm::DbConn;
use secrecy::Secret;
use tracing_subscriber::fmt::format;

use crate::auth::{validate_credentials, Credentials, AuthError};
use crate::session_state::TypedSession;
use crate::db::*;


#[tracing::instrument( name = "Assets", skip_all)]
#[get("/assets")]
pub async fn assets(db: web::Data<DbConn>) -> impl Responder {
    let credentials = Credentials {
        username: "admin".to_string(),
        password: Secret::new("everythinghastostartsomewhere".to_string())
    };

    let auth_text = match validate_credentials(credentials, &db).await {
        Ok(user_id) => {
            let mut text = format!("user id: {} found for username: admin\n", user_id);

            let user_roles = find_user_roles(user_id, &db).await;
            match user_roles {
                Ok(ur_vec) => {
                    for (user, role) in ur_vec {
                        match role {
                            Some(r) => {
                                let line = format!("\trole: {}\n", r.id);
                                text.push_str(line.as_str());
                            }
                            None => {  }
                        }
                    }
                }
                Err(_) => { 

                }
            }

            text
        }
        Err(e) => {
            match e {
                AuthError::InvalidCredentials(_) => "invalid credentials\n".to_string(),
                AuthError::UnexpectedError(_) => "auth unexepected error\n".to_string(),
            }
        }
    };



    //==============================================
    let assets = find_all_assets(&db).await;

    let mut body = "".to_string();
    body.push_str(auth_text.as_str());

    if let Ok(assets) = assets {
        for asset in &assets {
            let line = format!("{} - {}\n", asset.name, asset.description);
            body.push_str(line.as_str());
        }
    }
    else {
        body = "no assets found!".to_string();
    }

    body
}