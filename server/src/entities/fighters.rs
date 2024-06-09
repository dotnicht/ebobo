//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "fighters")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub fingerprint: String,
    #[sea_orm(unique)]
    pub emo: String,
    pub rank: i32,
    pub queued: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::matches::Entity")]
    Matches,
    #[sea_orm(has_many = "super::plays::Entity")]
    Plays,
}

impl Related<super::matches::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Matches.def()
    }
}

impl Related<super::plays::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Plays.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}