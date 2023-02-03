use sea_orm_migration::{prelude::*, sea_orm::{ActiveModelTrait, Set, NotSet, prelude::DateTimeLocal}};
use ::entity::{test_user, test_role};


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        let db = manager.get_connection();
        // create test roles
        let admin_role = test_role::ActiveModel {
            id: NotSet,
            name: Set("admin".to_string()),
            description: Set("default admin role".to_owned())
        }
        .insert(db)
        .await?;

        test_role::ActiveModel {
            id: NotSet,
            name: Set("teacher".to_string()),
            description: Set("teacher role has read only access".to_owned())
        }
        .insert(db)
        .await?;

        test_user::ActiveModel {
            id: NotSet,
            name: Set("admin".to_string()),
            created_on: Set(chrono::offset::Local::now().naive_local()),
            role_id: Set(Some(admin_role.id)),
        }
        .insert(db)
        .await?;


        Ok(()) 
    }
}
