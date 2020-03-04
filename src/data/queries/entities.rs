use anyhow::Result;

use crate::data::Repo;
use crate::data::models;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub fn insert(repo: &Repo, entity: models::entities::NewEntity) -> Result<models::entities::Entity> {
  use models::entities::Entity;

  let uuid = entity.id.as_bytes();
  let mut position = vec![];
  position.write_i32::<LittleEndian>(entity.pos.0)?;
  position.write_i32::<LittleEndian>(entity.pos.1)?;

  repo.conn().insert(uuid, position)?;

  Ok(Entity { id: entity.id, pos: entity.pos })
}

// this function is almost the same implementation as the insert function,
// but if continue more implemention,
// this function need transactional operation in the future.
// for that reason, I'll keep it for now.
pub fn update(repo: &Repo, entity: models::entities::UpdateEntity) -> Result<models::entities::Entity> {
    use models::entities::Entity;

    let uuid = entity.id.as_bytes();
    let mut position = vec![];
    position.write_i32::<LittleEndian>(entity.pos.0)?;
    position.write_i32::<LittleEndian>(entity.pos.1)?;

    repo.conn().insert(uuid, position)?;

    Ok(Entity { id: entity.id, pos: entity.pos })
}
