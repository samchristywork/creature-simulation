use crate::creature::Action;
use rand::seq::SliceRandom;
use std::fmt;

macro_rules! record_field_names {
    (
        pub struct $struct_name:ident {
            $(pub $key:ident : $type:ty),*
        }) => {

        #[derive(Clone, Copy)]
        pub struct $struct_name {
            $(pub $key : $type),*
        }

        #[derive(Debug)]
        enum TraitSetType {
            $(
                #[allow(non_camel_case_types)]
                $key
                ),*
        }

        impl $struct_name {
            #[allow(dead_code)]
            fn get_fields() -> &'static [&'static str] {
                static KEYS: &'static [&'static str] = &[$(stringify!($key)),*];
                KEYS
            }

            fn get_random_enum() -> &'static TraitSetType {
                [
                    $(TraitSetType::$key),*
                ].choose(&mut rand::thread_rng()).unwrap()
            }
        }
    }
}

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
        let action_pattern: [Action; 5] =
            [1; 5].map(|_| *actions.choose(&mut rand::thread_rng()).unwrap());
        Self { action_pattern }
    }
}

#[derive(Clone, Copy)]
pub struct Trait {
    pub value: i32,
    weight: f64,
}

impl Trait {
    fn new(value: i32, weight: f64) -> Self {
        Self { value, weight }
    }

    pub fn get_value(&self) -> f64 {
        self.value as f64 * self.weight
    }

    pub fn set_value(&mut self, value: i32) -> bool {
        if !(1..=10).contains(&value) {
            return false;
        }

        self.value = value;
        true
    }
}

record_field_names! {
pub struct TraitSet {
    pub aging_speed_divisor: Trait,
    pub eating_efficiency: Trait
}
}

#[derive(Clone, Copy)]
pub struct Genome {
    pub trait_set: TraitSet,
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
            trait_set: TraitSet {
                aging_speed_divisor: Trait::new(5, 0.2),
                eating_efficiency: Trait::new(5, 50.0),
            },
            behavior: Behavior::new(),
        }
    }

    fn trait_modify(&mut self, t: &TraitSetType, n: i32) -> bool {
        match t {
            TraitSetType::aging_speed_divisor => self
                .trait_set
                .aging_speed_divisor
                .set_value(self.trait_set.aging_speed_divisor.value + n),
            TraitSetType::eating_efficiency => self
                .trait_set
                .eating_efficiency
                .set_value(self.trait_set.eating_efficiency.value + n),
        }
    }

    /*
     * This function only commits changes if both modifications succeed.
     */
    fn trait_modify_duo(&mut self, a: &TraitSetType, b: &TraitSetType) {
        if self.trait_modify(a, -1) && !self.trait_modify(b, 1) {
            self.trait_modify(a, 1);
        }
    }

    pub fn mutate(&mut self) {
        self.trait_modify_duo(TraitSet::get_random_enum(), TraitSet::get_random_enum());
    }
}
