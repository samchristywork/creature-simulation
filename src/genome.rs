#[derive(Clone, Copy)]
pub struct Genome {
    pub aging_speed: i32,
}

impl Genome {
    pub fn new() -> Self {
        Self { aging_speed: 4 }
    }
}
