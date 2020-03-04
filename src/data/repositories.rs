use anyhow::Result;
use sled::Config;

use crate::data;
use crate::domain;
use crate::data::queries;

pub struct Repository {
    connection: sled::Db
}

impl Repository {
    pub fn new(&self) -> Result<Repository> {
        let config = Config::new().temporary(true);
        Ok(Repository { connection: config.open()? })
    }

    pub fn conn(&self) -> &sled::Db {
        &self.connection
    }
}

impl domain::repositories::Repository for Repository {
    fn create_entity(&self, entity: domain::models::entities::Entity) -> Result<domain::models::entities::Entity> {
        use data::models::entities::NewEntity;

        let entity = NewEntity{ id: entity.id, pos: entity.pos };
        let result = queries::entities::insert(&self, entity)?;

        Ok(domain::models::entities::Entity { id: result.id, pos: result.pos })
    }

    fn update_pos(&self, entity: domain::models::entities::Entity, pos: (i32, i32)) -> Result<domain::models::entities::Entity> {
        use data::models::entities::UpdateEntity;

        let entity = UpdateEntity{ id: entity.id, pos: pos };
        let result = queries::entities::update(&self, entity)?;

        Ok(domain::models::entities::Entity { id: result.id, pos: result.pos })
    }
}
