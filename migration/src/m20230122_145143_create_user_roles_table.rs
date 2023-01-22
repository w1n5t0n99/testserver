use sea_orm_migration::prelude::*;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserRoles::UserID).uuid().not_null())
                    .col(ColumnDef::new(UserRoles::RoleID).text().not_null())
                    .primary_key(Index::create().col(UserRoles::UserID).col(UserRoles::RoleID))
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_userroles_role")
                            .from(UserRoles::Table, UserRoles::RoleID)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_userroles_user")
                            .from(UserRoles::Table, UserRoles::UserID)
                            .to(User::Table, User::UserId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRoles::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum UserRoles {
    Table,
    UserID,
    RoleID,
}

#[derive(Iden)]
enum Role {
    Table,
    Id,
}

#[derive(Iden)]
enum User {
    Table,
    UserId,
}
