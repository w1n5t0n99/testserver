use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, QueryFilter};
use ::entity::{user, user_roles};
use ::entity::prelude::{User, UserRoles};



static ROLES: [&'static str; 5] = ["admin", "user_read", "user_edit", "user_create", "user_apply_action"];

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let admin = User::find()
            .filter(user::Column::Username.eq("admin"))
            .one(db)
            .await?
            .ok_or(DbErr::Migration("Migration Error - could not find admin user".to_string()))?;


        for r in ROLES {
            user_roles::ActiveModel {
                user_id: Set(admin.user_id),
                role_id: Set(r.to_owned()), 
            }
            .insert(db)
            .await?;
        }

        Ok(())
    }
}

