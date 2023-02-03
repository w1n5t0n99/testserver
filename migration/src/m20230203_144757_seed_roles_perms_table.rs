use entity::roles_permissions;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, QueryFilter};
use ::entity::{test_role};
use ::entity::prelude::{TestRole, RolesPermissions};


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let admin_role = TestRole::find()
            .filter(test_role::Column::Name.eq("admin"))
            .one(db)
            .await?
            .ok_or(DbErr::Migration("Migration Error - could not find admin user".to_string()))?;


        roles_permissions::ActiveModel {
            perm_id: Set("read_assets".to_string()),
            role_id: Set(admin_role.id), 
        }
        .insert(db)
        .await?;

        roles_permissions::ActiveModel {
            perm_id: Set("write_assets".to_string()),
            role_id: Set(admin_role.id), 
        }
        .insert(db)
        .await?;

        roles_permissions::ActiveModel {
            perm_id: Set("edit_assets".to_string()),
            role_id: Set(admin_role.id), 
        }
        .insert(db)
        .await?;

        let teacher_role = TestRole::find()
            .filter(test_role::Column::Name.eq("teacher"))
            .one(db)
            .await?
            .ok_or(DbErr::Migration("Migration Error - could not find admin user".to_string()))?;

        roles_permissions::ActiveModel {
            perm_id: Set("read_assets".to_string()),
            role_id: Set(teacher_role.id), 
        }
        .insert(db)
        .await?;

        Ok(())
    }
}

