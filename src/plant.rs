use crate::position::Position;

#[derive(Clone)]
pub struct Plant {
    pub position: Position,
}

impl Plant {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}
