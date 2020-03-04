use uuid::*;

pub struct Entity {
    pub id: Uuid,
    pub pos: (i32, i32)
}

impl Entity {
    fn new(id: Uuid, pos: (i32, i32)) -> Entity {
        Entity{ id: id, pos: pos }
    }
}
