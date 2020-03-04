use anyhow::Result;
use uuid::Uuid;

use crate::domain::models::entities::*;

pub trait Repository {
    fn create_entity(&self, entity: Entity) -> Result<Entity>;
    fn select_entity(&self, id: Uuid) -> Result<Entity>;
    fn update_entity(&self, id: Uuid, pos: (i32, i32)) -> Result<Entity>;
    fn delete_entity(&self, entity: Entity) -> Result<()>;
}
