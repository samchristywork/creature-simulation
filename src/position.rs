use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Sub;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn read(&self) {
        println!("{} {}", self.x, self.y);
    }

    pub fn rand(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn length(&self) -> f64 {
        (self.x as f64 * self.x as f64 + self.y as f64 * self.y as f64).sqrt()
    }

    pub fn dist(&self, position: &Position) -> f64 {
        let diff = *position - *self;
        diff.length()
    }
}
