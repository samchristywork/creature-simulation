use crate::creature::Creature;
use crate::map::Map;
use crate::plant::Plant;
use crate::position::Position;
use crate::terminal_graphics;
use crate::terminal_graphics::Continuation;
use crate::DisplayMode;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

use rand::Rng;

#[derive(Clone)]
pub struct WorldState {
    pub creatures: Vec<Creature>,
    pub plants: Vec<Plant>,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            creatures: Vec::new(),
            plants: Vec::new(),
        }
    }
}

pub struct World {
    pub name: String,
    pub history: Vec<WorldState>,
    pub current_state: WorldState,
    width: i32,
    height: i32,
}

impl World {
    pub fn new(width: i32, height: i32, name: String) -> Self {
        Self {
            name,
            history: Vec::new(),
            current_state: WorldState::new(),
            width,
            height,
        }
    }

    pub fn add_creatures(&mut self, n: i32) {
        for _ in 0..n {
            let creature = Creature::new(self.width / 2, self.height / 2);
            self.current_state.creatures.push(creature);
        }
    }

    pub fn add_plants(&mut self, n: i32) {
        for _ in 0..n {
            let x = rand::thread_rng().gen_range(0..self.width);
            let y = rand::thread_rng().gen_range(0..self.height);
            let plant = Plant::new(Position::new(x, y));
            self.current_state.plants.push(plant);
        }
    }

    pub fn simulate(&mut self, n: i32) {
        for _ in 0..n {
            self.step();
        }
    }

    fn step(&mut self) {
        self.history.push(self.current_state.clone());
        for creature in self.current_state.creatures.iter_mut() {
            creature.step();
        }
    }

    pub fn display_map(&self, mode: DisplayMode, states: &Vec<WorldState>) {
        if mode == DisplayMode::TerminalStatic {
            let state = &states[0];
            let mut map = Map::new(self.width, self.height, self.name.to_string());
            for creature in &state.creatures {
                map.set_creature(creature.position);
            }
            for plant in &state.plants {
                map.set_plant(plant.position);
            }
        } else if mode == DisplayMode::TerminalDynamic {
            enable_raw_mode().unwrap();
            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen).unwrap();
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend).unwrap();

            for state in states {
                let mut map = Map::new(self.width, self.height, self.name.to_string());
                for creature in &state.creatures {
                    map.set_creature(creature.position);
                }
                for plant in &state.plants {
                    map.set_plant(plant.position);
                }
                let cont = terminal_graphics::run(&mut terminal, &map);
                if cont == Continuation::Halt {
                    break;
                }
            }

            disable_raw_mode().unwrap();
            execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
            terminal.show_cursor().unwrap();
        }
    }
}
