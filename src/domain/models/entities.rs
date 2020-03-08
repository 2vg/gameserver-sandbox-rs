#[derive(Clone, Debug)]
pub struct Entity {
    pub id: u32,
    pub pos: (i32, i32),
}

impl Entity {
    pub fn new_with_empty() -> Entity {
        Entity {
            id: rand::random::<u32>(),
            pos: (0, 0),
        }
    }

    pub fn new(id: u32, pos: (i32, i32)) -> Entity {
        Entity { id: id, pos: pos }
    }
}
