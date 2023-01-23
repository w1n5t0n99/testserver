use sea_orm::{RelationDef, Linked, RelationTrait};

#[derive(Debug)]
pub struct UserToRole;

impl Linked for UserToRole {
    type FromEntity = super::user::Entity;

    type ToEntity = super::role::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            super::user_roles::Relation::User.def().rev(),
            super::user_roles::Relation::Role.def(),
        ]
    }
}