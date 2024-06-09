//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "plays")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub fighter: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub r#match: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::fighters::Entity",
        from = "Column::Fighter",
        to = "super::fighters::Column::Fingerprint",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Fighters,
}

impl Related<super::fighters::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Fighters.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}