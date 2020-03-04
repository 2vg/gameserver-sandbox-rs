use anyhow::{Result, bail};
use uuid::Uuid;

use crate::data::models::entities::*;
use crate::data::repositories::Repository;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub fn insert(repo: &Repository, entity: NewEntity) -> Result<Entity> {
    let uuid = entity.id.as_bytes();
    let mut position = vec![];
    position.write_i32::<LittleEndian>(entity.pos.0)?;
    position.write_i32::<LittleEndian>(entity.pos.1)?;

    repo.conn().insert(uuid, position)?;

    Ok(Entity { id: entity.id, pos: entity.pos })
}

pub fn select_one(repo: &Repository, id: Uuid) -> Result<Entity> {
    if let Some(result) = repo.conn().get(id.as_bytes())? {
      let mut position = Cursor::new(result);
      let pos_x = position.read_i32::<LittleEndian>()?;
      let pos_y = position.read_i32::<LittleEndian>()?;
      Ok(Entity { id: id, pos: (pos_x, pos_y) })
    }
    else {
      bail!("entity not found.")
    }
}

// this function is almost the same implementation as the insert function,
// but if continue more implemention,
// this function need transactional operation in the future.
// for that reason, I'll keep it for now.
pub fn update(repo: &Repository, entity: UpdateEntity) -> Result<Entity> {
    let uuid = entity.id.as_bytes();
    let mut position = vec![];
    position.write_i32::<LittleEndian>(entity.pos.0)?;
    position.write_i32::<LittleEndian>(entity.pos.1)?;

    repo.conn().insert(uuid, position)?;

    Ok(Entity { id: entity.id, pos: entity.pos })
}

pub fn delete(repo: &Repository, entity: Entity) -> Result<()> {
    let uuid = entity.id.as_bytes();

    repo.conn().remove(uuid)?;

    Ok(())
}
