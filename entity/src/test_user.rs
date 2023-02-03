//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "test_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_on: DateTime,
    pub role_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::test_role::Entity",
        from = "Column::RoleId",
        to = "super::test_role::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    TestRole,
}

impl Related<super::test_role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TestRole.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
