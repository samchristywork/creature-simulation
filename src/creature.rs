use crate::genome::Genome;
use crate::plant::Plant;
use crate::position::Position;
use rand::seq::SliceRandom;
use rand::Rng;
use std::slice::Iter;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    TurnLeft,
    TurnRight,
    MoveForward,
    RandomWalk,
}

impl Action {
    pub fn iterator() -> Iter<'static, Action> {
        static ACTION: [Action; 4] = [
            Action::TurnLeft,
            Action::TurnRight,
            Action::MoveForward,
            Action::RandomWalk,
        ];
        ACTION.iter()
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy)]
pub struct Creature {
    pub life: i32,
    pub direction: Direction,
    pub position: Position,
    genome: Genome,
}

impl Creature {
    pub fn new(x: i32, y: i32) -> Self {
        let directions = vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        let direction: Direction = *directions.choose(&mut rand::thread_rng()).unwrap();
        Self {
            life: 255,
            position: Position::new(x, y),
            direction,
            genome: Genome::new(),
        }
    }

    pub fn move_forward(&mut self) {
        match self.direction {
            Direction::North => self.move_relative(0, 1),
            Direction::South => self.move_relative(0, -1),
            Direction::East => self.move_relative(1, 0),
            Direction::West => self.move_relative(-1, 0),
        }
    }

    pub fn turn_right(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::South => self.direction = Direction::West,
            Direction::East => self.direction = Direction::South,
            Direction::West => self.direction = Direction::North,
        }
    }

    pub fn turn_left(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::South => self.direction = Direction::East,
            Direction::East => self.direction = Direction::North,
            Direction::West => self.direction = Direction::South,
        }
    }

    pub fn step(&mut self, plants: &[Plant]) {
        if self.life > 0 {
            for plant in plants {
                if plant.position == self.position {
                    self.life = 255;
                }
            }
            let actions = Action::iterator().as_slice();
            match *actions.choose(&mut rand::thread_rng()).unwrap() {
                Action::MoveForward => self.move_forward(),
                Action::TurnLeft => self.turn_left(),
                Action::TurnRight => self.turn_right(),
                Action::RandomWalk => self.random_walk(),
            }
            self.age();
        }
    }

    fn move_relative(&mut self, x: i32, y: i32) {
        self.position.x += x;
        self.position.y += y;
    }

    fn random_walk(&mut self) {
        if rand::thread_rng().gen_range(0..4) == 0 {
            self.move_relative(
                rand::thread_rng().gen_range(-1..2),
                rand::thread_rng().gen_range(-1..2),
            );
        }
    }

    fn age(&mut self) {
        let n = self.genome.aging_speed;
        if self.life < n {
            self.life = 0;
            return;
        }
        self.life -= n;
    }
}
