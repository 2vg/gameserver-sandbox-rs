use crate::domain::repositories::Repository;

pub struct Context<R: Repository> {
    pub repository: R
}
