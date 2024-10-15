use sea_orm::entity::prelude::*;
use super::_entities::songs::{ActiveModel, Entity};
pub type Songs = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
