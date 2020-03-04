use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct Entity {
    pub id: Uuid,
    pub pos: (i32, i32),
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

pub struct NewEntity {
    pub id: Uuid,
    pub pos: (i32, i32)
}

pub struct UpdateEntity {
    pub id: Uuid,
    pub pos: (i32, i32)
}
