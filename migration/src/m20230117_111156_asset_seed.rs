use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{entity::*, query::*};


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(Asset::Table)
            .columns([Asset::Name, Asset::Description])
            .values_panic(["RHS-LAP-400".into(), "Dell Laptop".into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

}

#[derive(Iden)]
enum Asset {
    Table,
    Id,
    Name,
    Description,
}
