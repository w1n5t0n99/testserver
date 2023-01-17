use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            sea_query::Table::create()
            .table(Asset::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Asset::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key()
            )
            .col(ColumnDef::new(Asset::Name).string().not_null())
            .col(ColumnDef::new(Asset::Description).string().not_null())
            .to_owned()
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Asset::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Asset {
    Table,
    Id,
    Name,
    Description,
}
