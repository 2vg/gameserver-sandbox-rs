use anyhow::Result;

use crate::domain::models::*;
use crate::domain::repositories::Repository;
use crate::app::server::*;

pub fn run_example_app<R: Repository>(app: App<R>) -> Result<()> {
    let ent = entities::Entity::new_with_empty();

    let result = app.ctx.repository.create_entity(ent)?;
    println!("Ent: id={}, pos={:?}", result.id, result.pos);

    Ok(())
}
