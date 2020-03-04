extern crate gameserver_sandbox_rs as sandbox;

use anyhow::Result;

use sandbox::domain::models::entities::Entity;
use sandbox::data::repositories::Repository;

use sandbox::app::server;
use sandbox::app::example;

fn main() -> Result<()> {
    let repo = Repository::new()?;
    let example_app = server::new_app(repo);

    example::run_example_app(example_app);

    Ok(())
}
