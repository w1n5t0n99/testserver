use sea_orm_migration::prelude::*;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RolesPermissions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RolesPermissions::RoleID).integer().not_null())
                    .col(ColumnDef::new(RolesPermissions::PermID).text().not_null())
                    .primary_key(Index::create().col(RolesPermissions::RoleID).col(RolesPermissions::PermID))
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_rolespermissions_perms")
                            .from(RolesPermissions::Table, RolesPermissions::PermID)
                            .to(Permissions::Table, Permissions::Name)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_rolespermissions_roles")
                            .from(RolesPermissions::Table, RolesPermissions::RoleID)
                            .to(TestRole::Table, TestRole::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolesPermissions::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum RolesPermissions {
    Table,
    RoleID,
    PermID,
}

#[derive(Iden)]
enum TestRole {
    Table,
    Id,
}

#[derive(Iden)]
enum Permissions {
    Table,
    Name,
}
