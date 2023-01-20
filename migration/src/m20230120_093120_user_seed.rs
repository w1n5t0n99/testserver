use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, query::*};
use ::entity::{prelude::Asset, asset, prelude::User, user};


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        user::ActiveModel {
            user_id: Set(uuid::Uuid::new_v4()),
            username: Set("admin".to_owned()),
            password_hash: Set("$argon2id$v=19$m=15000,t=2,p=1$OEx/rcq+3ts//WUDzGNl2g$Am8UFBA4w5NJEmAtquGvBmAlu92q/VQcaoL5AyJPfc8".to_owned()),
        }
        .insert(db)
        .await?;
        
        Ok(())
    }
}

