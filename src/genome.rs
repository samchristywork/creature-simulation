use crate::creature::Action;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone, Copy)]
struct Behavior {
    action_pattern: [Action; 10],
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
pub struct Genome {
    pub aging_speed: i32,
    eating_efficiency: f32,
    behavior: Behavior,
}

impl Genome {
    pub fn new() -> Self {
        Self {
            aging_speed: rand::thread_rng().gen_range(1..4),
            eating_efficiency: 1.0,
            behavior: Behavior::new(),
        }
    }
}
