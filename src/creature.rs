use crate::position::Position;

use rand::Rng;

#[derive(Clone)]
pub struct Creature {
    life: i32,
    dead: bool,
    pub position: Position,
}

impl Creature {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            life: 100,
            dead: false,
            position: Position::new(x, y),
        }
    }

    pub fn step(&mut self) {
        self.age();
        self.random_walk();
    }

    fn move_relative(&mut self, x: i32, y: i32) {
        self.position.x += x;
        self.position.y += y;
    }

    fn random_walk(&mut self) {
        self.move_relative(
            rand::thread_rng().gen_range(-1..2),
            rand::thread_rng().gen_range(-1..2),
        );
    }

    fn age(&mut self) {
        self.life -= 1;
        if self.life <= 0 {
            self.dead = true;
        }
    }

    pub fn display_position(&self) {
        println!("{} {}", self.position.x, self.position.y);
    }
}
