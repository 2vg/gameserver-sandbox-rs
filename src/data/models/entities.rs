//use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: u32,
    pub pos: (i32, i32),
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

pub struct NewEntity {
    pub id: u32,
    pub pos: (i32, i32),
}

pub struct UpdateEntity {
    pub id: u32,
    pub pos: (i32, i32),
}
