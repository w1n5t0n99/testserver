use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::*;
use ::entity::permissions;


static ROLES: [&'static str; 5] = ["admin", "user_read", "user_edit", "user_create", "user_apply_action"];

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        permissions::ActiveModel {
            name: Set("read_assets".to_string()),
            description: Set("Permission to read asset items group".to_string())
        }
        .insert(db)
        .await?;

        permissions::ActiveModel {
            name: Set("write_assets".to_string()),
            description: Set("Permission to write asset items group".to_string())
        }
        .insert(db)
        .await?;

        permissions::ActiveModel {
            name: Set("edit_assets".to_string()),
            description: Set("Permission to edit asset items group".to_string())
        }
        .insert(db)
        .await?;
        
        Ok(())
    }
}

