use crate::genome::Genome;
use crate::plant::Plant;
use crate::position::Position;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;
use std::slice::Iter;

#[derive(Clone, Copy, Debug)]
pub enum Action {
    MoveForward,
    TurnLeft,
    TurnRandom,
    TurnRight,
}

impl Action {
    pub fn iterator() -> Iter<'static, Action> {
        static ACTION: [Action; 4] = [
            Action::MoveForward,
            Action::TurnLeft,
            Action::TurnRandom,
            Action::TurnRight,
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
    name: [char; 15],
    pub life: f64,
    pub direction: Direction,
    pub position: Position,
    genome: Genome,
    program_counter: usize,
    pub world_bounds: Position,
}

fn array_from_str(string: &str) -> [char; 15] {
    let mut ret: [char; 15] = [' '; 15];
    let bytes = string.as_bytes();
    for i in 0..15 {
        if i < bytes.len() {
            ret[i] = bytes[i] as char;
        } else {
            ret[i] = ' ';
        }
    }
    ret
}

fn string_from_array(string: [char; 15]) -> String {
    let mut ret = String::new();
    for i in 0..15 {
        ret += string[i].to_string().as_str();
    }
    ret
}

impl fmt::Display for Creature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{2} {1} {0}",
            string_from_array(self.name).trim(),
            self.life,
            self.genome
        )
    }
}

impl Creature {
    pub fn new(x: i32, y: i32, world_bounds: Position, name: &str) -> Self {
        let directions = vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];
        let direction: Direction = *directions.choose(&mut rand::thread_rng()).unwrap();
        Self {
            name: array_from_str(name),
            life: 255.0,
            position: Position::new(x, y),
            direction,
            genome: Genome::new_even_distribution(),
            program_counter: 0,
            world_bounds,
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
        if self.life > 0.0 {
            for plant in plants {
                if plant.position == self.position {
                    self.life = self.genome.eating_efficiency.get_value() + self.life;
                    if self.life > 255.0 {
                        self.life = 255.0;
                    }
                }
            }
            let action = self.genome.behavior.action_pattern[self.program_counter];
            match action {
                Action::MoveForward => self.move_forward(),
                Action::TurnLeft => self.turn_left(),
                Action::TurnRandom => self.random_turn(),
                Action::TurnRight => self.turn_right(),
            }
            self.program_counter += 1;
            if self.program_counter > 9 {
                self.program_counter = 0;
            }
            self.age();
        }
    }

    pub fn move_relative(&mut self, x: i32, y: i32) {
        self.position.x += x;
        self.position.y += y;

        if self.position.x > self.world_bounds.x {
            self.position.x = 0;
        }
        if self.position.x < 0 {
            self.position.x = self.world_bounds.x;
        }
        if self.position.y > self.world_bounds.y {
            self.position.y = 0;
        }
        if self.position.y < 0 {
            self.position.y = self.world_bounds.y;
        }
    }

    fn random_turn(&mut self) {
        if rand::thread_rng().gen_range(0..2) == 0 {
            self.turn_right();
        } else {
            self.turn_left();
        }
    }

    fn age(&mut self) {
        let n = 1.0 / self.genome.aging_speed_divisor.get_value() + 1.0;
        if self.life < n {
            self.life = 0.0;
            return;
        }
        self.life -= n;
    }
}
