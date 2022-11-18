use crate::creature::Direction;
use crate::position::Position;
use colored::Colorize;

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub slots: Vec<Vec<(char, i32)>>,
    pub name: String,
}

impl Map {
    pub fn new(width: usize, height: usize, name: String) -> Self {
        let slots = vec![vec![(' ', 0); width.try_into().unwrap()]; height.try_into().unwrap()];
        Self {
            width,
            height,
            slots,
            name,
        }
    }

    pub fn set_creature(&mut self, position: Position, direction: Direction, life: i32) {
        if life == 0 {
            self.set_slot(position, 'x', 1);
            return;
        }
        match direction {
            Direction::North => self.set_slot(position, '^', life),
            Direction::South => self.set_slot(position, 'v', life),
            Direction::East => self.set_slot(position, '>', life),
            Direction::West => self.set_slot(position, '<', life),
        }
    }

    pub fn set_plant(&mut self, position: Position) {
        self.set_slot(position, '.', 2);
    }

    fn get_slot(&mut self, position: Position) -> (char, i32) {
        if position.x >= 0 && position.x < self.width as i32 {
            if position.y >= 0 && position.y < self.height as i32 {
                return self.slots[position.y as usize][position.x as usize];
            }
        }
        (' ', 0)
    }

    fn set_slot(&mut self, position: Position, character: char, life: i32) {
        if position.x >= 0 && position.x < self.width as i32 {
            if position.y >= 0 && position.y < self.height as i32 {
                let curr = self.get_slot(position).1;
                if life > curr {
                    self.slots[position.y as usize][position.x as usize] = (character, life);
                }
            }
        }
    }

    pub fn display(&self) {
        print!(" ");
        for _ in 0..self.width {
            print!("_");
        }
        println!();
        for x in &self.slots {
            print!("|");
            for y in x {
                if y.0 == '.' {
                    let s = format!("{}", y.0).green();
                    print!("{}", s);
                } else if y.0 == 'c' {
                    let s = format!("{}", y.0).yellow();
                    print!("{}", s);
                } else {
                    print!("{}", y.0);
                }
            }
            println!("|");
        }
        print!(" ");
        for _ in 0..self.width {
            print!("_");
        }
        println!();
    }
}
