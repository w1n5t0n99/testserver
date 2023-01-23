use sea_orm::*;
use ::entity::{asset, user, role};
use ::entity::prelude::{Asset, User, Role};
use ::entity::entity_linked;
use secrecy::{Secret, ExposeSecret};

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

pub async fn find_user_roles(user_id: uuid::Uuid, db: &DbConn) -> Result<Vec<(user::Model, Option<role::Model>)>, DbErr> {
    let users = User::find_by_id(user_id)
        .find_also_linked(entity_linked::UserToRole)
        .all(db)
        .await?;

    Ok(users)
}