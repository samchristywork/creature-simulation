use crate::creature::Action;
use rand::seq::SliceRandom;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Behavior {
    pub action_pattern: [Action; 5],
}

impl fmt::Display for Behavior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        for action in self.action_pattern {
            res += match action {
                Action::MoveForward => "F",
                Action::TurnLeft => "L",
                Action::TurnRight => "R",
                Action::TurnRandom => "A",
            }
        }
        write!(f, "{}", res)
    }
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
    pub aging_speed_divisor: Trait,
    pub eating_efficiency: Trait,
    pub behavior: Behavior,
}

impl fmt::Display for Genome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.behavior)
    }
}

impl Genome {
    pub fn new_even_distribution() -> Self {
        Self {
            aging_speed_divisor: Trait::new(5, 0.2),
            eating_efficiency: Trait::new(5, 50.0),
            behavior: Behavior::new(),
        }
    }
}
