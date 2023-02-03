//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    pub password_hash: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_roles::Relation::Role.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_roles::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
