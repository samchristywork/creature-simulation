use crate::creature::Creature;
use crate::map::Map;
use crate::plant::Plant;
use crate::position::Position;

use rand::Rng;

pub struct World {
    pub name: String,
    creatures: Vec<Creature>,
    plants: Vec<Plant>,
    width: i32,
    height: i32,
}

impl World {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            name: "hi".to_string(),
            creatures: Vec::new(),
            plants: Vec::new(),
            width,
            height,
        }
    }
    pub fn add_creatures(&mut self, n: i32) {
        for _ in 0..n {
            let creature = Creature::new(self.width / 2, self.height / 2);
            self.creatures.push(creature);
        }
    }
    pub fn add_plants(&mut self, n: i32) {
        for _ in 0..n {
            let x = rand::thread_rng().gen_range(0..self.width);
            let y = rand::thread_rng().gen_range(0..self.height);
            let plant = Plant::new(Position::new(x, y));
            self.plants.push(plant);
        }
    }
    pub fn simulate(&mut self, n: i32) {
        for _ in 0..n {
            self.step();
        }
    }
    fn step(&mut self) {
        for creature in self.creatures.iter_mut() {
            creature.step();
        }
    }
    pub fn display_results(&self) {
        self.display_map();
    }
    pub fn display_map(&self) {
        let mut map = Map::new(self.width, self.height);
        for creature in &self.creatures {
            map.set_creature(creature.position);
        }
        for plant in &self.plants {
            map.set_plant(plant.position);
        }
        map.display();
    }
}
