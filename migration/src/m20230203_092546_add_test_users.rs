use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TestUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TestUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TestUser::Name).string().not_null())
                    .col(ColumnDef::new(TestUser::CreatedOn).date_time().not_null())
                    .col(ColumnDef::new(TestUser::RoleID).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_role_user_id")
                            .from(TestUser::Table, TestUser::RoleID)
                            .to(TestRole::Table, TestRole::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TestUser::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum TestUser {
    Table,
    Id,
    Name,
    CreatedOn,
    RoleID,
}

#[derive(Iden)]
enum TestRole {
    Table,
    Id,
}