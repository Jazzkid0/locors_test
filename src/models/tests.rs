use sea_orm::entity::prelude::*;
use super::_entities::tests::{ActiveModel, Entity};
pub type Tests = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
