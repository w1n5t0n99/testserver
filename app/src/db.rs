use sea_orm::*;
use ::entity::{prelude::Asset, asset, prelude::User, user};


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