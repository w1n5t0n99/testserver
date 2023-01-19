pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_asset_table;
mod m20230117_111156_asset_seed;
mod m20230119_102725_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_asset_table::Migration),
            Box::new(m20230117_111156_asset_seed::Migration),
            Box::new(m20230119_102725_create_user_table::Migration),
        ]
    }
}
