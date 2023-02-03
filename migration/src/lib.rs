pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_asset_table;
mod m20230117_111156_asset_seed;
mod m20230119_102725_create_user_table;
mod m20230120_093120_user_seed;
mod m20230122_141730_create_roles_table;
mod m20230122_145143_create_user_roles_table;
mod m20230122_185614_role_seed;
mod m20230122_190717_admin_roles_seed;
mod m20230203_085353_add_test_roles;
mod m20230203_092546_add_test_users;
mod m20230203_102343_seed_test_user;
mod m20230203_142254_create_perms_table;
mod m20230203_143026_seed_perms_table;
mod m20230203_143748_create_roles_perms_table;
mod m20230203_144757_seed_roles_perms_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_asset_table::Migration),
            Box::new(m20230117_111156_asset_seed::Migration),
            Box::new(m20230119_102725_create_user_table::Migration),
            Box::new(m20230120_093120_user_seed::Migration),
            Box::new(m20230122_141730_create_roles_table::Migration),
            Box::new(m20230122_145143_create_user_roles_table::Migration),
            Box::new(m20230122_185614_role_seed::Migration),
            Box::new(m20230122_190717_admin_roles_seed::Migration),
            Box::new(m20230203_085353_add_test_roles::Migration),
            Box::new(m20230203_092546_add_test_users::Migration),
            Box::new(m20230203_102343_seed_test_user::Migration),
            Box::new(m20230203_142254_create_perms_table::Migration),
            Box::new(m20230203_143026_seed_perms_table::Migration),
            Box::new(m20230203_143748_create_roles_perms_table::Migration),
            Box::new(m20230203_144757_seed_roles_perms_table::Migration),
        ]
    }
}
