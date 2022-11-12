use crate::position::Position;

pub struct Plant {
    pub position: Position,
}

impl Plant {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}
