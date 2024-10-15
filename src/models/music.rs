use sea_orm::entity::prelude::*;
use super::_entities::music::{ActiveModel, Entity};
pub type Music = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
