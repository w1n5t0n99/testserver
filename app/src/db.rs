use anyhow::Context;
use sea_orm::*;
use ::entity::{asset, user, role, test_role, test_user};
use ::entity::prelude::{Asset, User, Role, TestRole, TestUser};
use ::entity::entity_linked;
use secrecy::{Secret, ExposeSecret};
use serde::Deserialize;

use crate::filesystem::FilePayload;
use crate::utils::{error_chain_fmt, spawn_blocking_with_tracing};

pub async fn find_assets_in_page(
    db: &DbConn,
    page: u64,
    items_per_page: u64
) -> Result<(Vec<asset::Model>, u64), DbErr> {
    let paginator = Asset::find()
        .order_by_asc(asset::Column::Id)
        .paginate(db, items_per_page);

    let num_pages = paginator.num_pages().await?;

    paginator.fetch_page(page-1).await.map(|p| (p, num_pages))
}

pub async fn find_all_assets(db: &DbConn) -> Result<Vec<asset::Model>, DbErr> {
    let assets = Asset::find().all(db).await;
    assets
}

pub async fn find_user(username: &str, db: &DbConn) -> Result<Option<user::Model>, DbErr> {
    User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
}

pub async fn update_user_password(user_id: uuid::Uuid, password_hash: Secret<String>, db: &DbConn) -> Result<(), DbErr> {
    let user = User::find_by_id(user_id).one(db).await?;
    let mut user: user::ActiveModel = user.unwrap().into();
    
    user.password_hash = Set(password_hash.expose_secret().to_owned());
    user.update(db).await?;    

    Ok(())
}

pub async fn find_user_roles(user_id: uuid::Uuid, db: &DbConn) -> Result<Vec<String>, DbErr> {
    let users = User::find_by_id(user_id)
        .find_also_linked(entity_linked::UserToRole)
        .all(db)
        .await?;

    let roles: Vec<String> = users.iter()
        .filter_map(|(_user, role)| 
            match role {
                Some(role) => Some(role.id.clone()),
                None => None,            
            }
        )
        .collect();

    Ok(roles)
}


pub async fn insert_asset(asset_model: asset::Model, db: &DbConn) -> Result<(), DbErr> {
    let mut asset_active: asset::ActiveModel = asset_model.into();
    asset_active.id = ActiveValue::NotSet;

    let _res = asset::Entity::insert(asset_active).exec(db).await?;

    Ok(())
}

#[derive(Debug, serde::Serialize)]
pub struct BulkInsert {
    pub total: usize,
    pub inserted: usize,
    pub skipped: usize,
}

#[derive(thiserror::Error)]
pub enum BulkInsertError {
    #[error("Error parsing payload file")]
    Parse(#[from] csv::Error),
    #[error("Error inserting in database")]
    Database(#[from] DbErr),
    #[error("Something went wrong")]
    Unexpected(#[from] anyhow::Error),
}

impl std::fmt::Debug for BulkInsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub async fn insert_assets_from_payload(payload: &FilePayload, db: &DbConn) -> Result<BulkInsert, BulkInsertError> {
    let mut rdr = csv::Reader::from_path(&payload.tmp_path)?;

    let assets = spawn_blocking_with_tracing(move || {
        let assets: Vec<Result<asset::Model, csv::Error>> = rdr.deserialize()
            .map(|row| {
                match row {
                    Ok(r) => { let a: asset::Model = r; Ok(a) }
                    Err(e) => Err(e),
                }
            })
            .collect();

            assets
    })
    .await
    .context("Blocking thread error")?;

    let total = assets.len();
    let mut inserted: usize = 0;
    let mut skipped: usize = 0;

    for a in assets {
        match a {
            Err(_) => { skipped += 1; }
            Ok(a) => {
                let res = insert_asset(a, db).await;
                match res {
                    Ok(_) => { inserted += 1; } 
                    Err(_) => {skipped += 1; }
                }
            }
        }
    }

    let tmp_path = payload.tmp_path.clone();
    spawn_blocking_with_tracing(move || std::fs::remove_file(tmp_path))
            .await
            .context("blocking thread error")?
            .context("tmp file delete error")?;

    Ok(BulkInsert { total, inserted, skipped })
}

pub async fn find_test_user_info(name: &str, db: &DbConn) -> Result<String, DbErr> {
    let role = TestUser::find()
        .filter(test_user::Column::Name.eq(name))
        .find_also_related(TestRole)
        .one(db)
        .await?;

    let msg = match role {
        None => "No TestUser found".to_string(),
        Some((user, role)) => {
            let role_name = match role {
                None => "No TestRole found".to_string(),
                Some(role) => format!("Role[ name: {} | description: {} ]", role.name, role.description)
            };

            format!("User[ name: {}] {}", user.name, role_name)            
        }
    };

    Ok(msg)
}