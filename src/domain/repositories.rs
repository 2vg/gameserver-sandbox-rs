use anyhow::Result;

use crate::domain::models::entities::*;

pub trait Repository {
    fn create_entity(&self, entity: Entity) -> Result<Entity>;
    fn update_pos(&self, entity: Entity, pos: (i32, i32)) -> Result<Entity>;
}
