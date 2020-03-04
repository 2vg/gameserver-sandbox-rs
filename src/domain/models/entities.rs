use uuid::*;

pub struct Entity {
    pub id: Uuid,
    pub pos: (i32, i32)
}

impl Entity {
    pub fn new_with_empty() -> Entity {
        Entity{ id: Uuid::new_v4(), pos: (0, 0) }
    }

    pub fn new(id: Uuid, pos: (i32, i32)) -> Entity {
        Entity{ id: id, pos: pos }
    }
}
