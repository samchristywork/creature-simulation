use crate::creature::Creature;
use crate::plant::Plant;

pub struct World {
    pub name: String,
    creatures: Vec<Creature>,
    plants: Vec<Plant>,
}

impl World {
    pub fn new() -> Self {
        Self {
            name: "hi".to_string(),
            creatures: Vec::new(),
            plants: Vec::new(),
        }
    }
    pub fn add_creatures(&mut self, n: i32) {
        for _ in 0..n {
            let creature = Creature::new(0, 0);
            self.creatures.push(creature);
        }
    }
    pub fn add_plants(&mut self, n: i32) {
        for _ in 0..n {
            let plant = Plant::new();
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
        self.creatures[0].display_history();
        //for creature in &self.creatures[0..10] {
        //    creature.display_position();
        //}
    }
}
