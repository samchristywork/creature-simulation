use crate::creature::Action;
use rand::seq::SliceRandom;

#[derive(Clone, Copy)]
pub struct Behavior {
    pub action_pattern: [Action; 10],
}

impl Behavior {
    fn new() -> Self {
        let actions = Action::iterator().as_slice();
        let action_pattern: [Action; 10] =
            [1; 10].map(|_| *actions.choose(&mut rand::thread_rng()).unwrap());
        Self { action_pattern }
    }
}

#[derive(Clone, Copy)]
pub struct Trait {
    value: i32,
    weight: f64,
}

impl Trait {
    fn new(value: i32, weight: f64) -> Self {
        Self { value, weight }
    }
    pub fn get_value(&self) -> f64 {
        self.value as f64 * self.weight
    }
}

#[derive(Clone, Copy)]
pub struct Genome {
    pub aging_speed: Trait,
    pub eating_efficiency: Trait,
    pub behavior: Behavior,
}

impl Genome {
    pub fn new_even_distribution() -> Self {
        Self {
            aging_speed: Trait::new(5, 0.2),
            eating_efficiency: Trait::new(5, 50.0),
            behavior: Behavior::new(),
        }
    }
}
