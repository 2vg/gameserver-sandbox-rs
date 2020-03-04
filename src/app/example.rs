use anyhow::*;

use crate::domain::models::*;
use crate::domain::repositories::Repository;
use crate::app::server::*;

pub fn run_example_app<R: Repository>(app: App<R>) -> anyhow::Result<()> {
    let ent = entities::Entity::new_with_empty();

    let result = app.ctx.repository.create_entity(ent)?;
    let (id, pos) = (result.id, result.pos);
    println!("Created Ent: id={}, pos={:?}", &id, &pos);

    let result = app.ctx.repository.update_entity(id, (10, 20))?;
    let (id, pos) = (result.id, result.pos);
    println!("Updated Ent: id={}, pos={:?}", &id, &pos);

    let ent = entities::Entity::new(id.clone(), pos);
    let result = app.ctx.repository.delete_entity(ent)?;
    println!("Ent was deleted.");

    let result = app.ctx.repository.select_entity(id)?;
    Ok(())
}
