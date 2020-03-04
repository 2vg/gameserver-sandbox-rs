use anyhow::Result;
use sled::Config;

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
