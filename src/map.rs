use crate::position::Position;
use colored::Colorize;

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub slots: Vec<Vec<char>>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        let slots = vec![vec![' '; width.try_into().unwrap()]; height.try_into().unwrap()];
        Self {
            width,
            height,
            slots,
        }
    }

    pub fn set_creature(&mut self, position: Position) {
        self.set_slot(position, 'c');
    }

    pub fn set_plant(&mut self, position: Position) {
        self.set_slot(position, '.');
    }

    fn set_slot(&mut self, position: Position, character: char) {
        if position.x >= 0 && position.x < self.width {
            if position.y >= 0 && position.y < self.height {
                self.slots[position.y as usize][position.x as usize] = character;
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
                if y == &'.' {
                    let s = format!("{}", y).green();
                    print!("{}", s);
                } else if y == &'c' {
                    let s = format!("{}", y).yellow();
                    print!("{}", s);
                } else {
                    print!("{}", y);
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
