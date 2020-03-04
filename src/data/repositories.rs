use anyhow::Result;

use crate::data;
use crate::domain;
use crate::data::queries;

pub struct Repository(pub data::Repo);

impl domain::repositories::Repository for Repository {
    fn create_entity(&self, entity: domain::models::entities::Entity) -> Result<domain::models::entities::Entity> {
        use data::models::entities::NewEntity;

        let entity = NewEntity{ id: entity.id, pos: entity.pos };
        let result = queries::entities::insert(&self.0, entity)?;

        Ok(domain::models::entities::Entity { id: result.id, pos: result.pos })
    }

    fn update_pos(&self, entity: domain::models::entities::Entity, pos: (i32, i32)) -> Result<domain::models::entities::Entity> {
        use data::models::entities::UpdateEntity;

        let entity = UpdateEntity{ id: entity.id, pos: pos };
        let result = queries::entities::update(&self.0, entity)?;

        Ok(domain::models::entities::Entity { id: result.id, pos: result.pos })
    }
}
