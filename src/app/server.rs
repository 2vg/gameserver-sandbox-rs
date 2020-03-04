use anyhow::Result;

use crate::domain::models::*;

use crate::app::context::Context;
use crate::domain::repositories::Repository;

pub struct App<R: Repository> {
    pub ctx: Context<R>
}

pub fn new_app<R: Repository>(repository: R) -> App<R> {
    let context = Context { repository };
    App { ctx: context }
}
