use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::*;
use ::entity::role;


static ROLES: [&'static str; 5] = ["admin", "user_read", "user_edit", "user_create", "user_apply_action"];

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        for r in ROLES {
            role::ActiveModel {
                id: Set(r.to_owned()),
            }
            .insert(db)
            .await?;
        }

        Ok(())
    }
}

