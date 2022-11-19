use crate::creature::Creature;
use crate::map::Map;
use crate::position::Position;
use crate::terminal_graphics;
use crate::terminal_graphics::Cursor;
use crate::terminal_graphics::Interaction;
use crate::DisplayMode;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

pub fn plant_is_here(position: Position) -> bool {
    (position.x + position.y) % 13 == 0
}

#[derive(Clone, Default)]
pub struct WorldState {
    pub creatures: Vec<Creature>,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            creatures: Vec::new(),
        }
    }

    pub fn get_creatures_at(&self, position: Position) -> Vec<&Creature> {
        let mut creatures = Vec::new();
        for creature in &self.creatures {
            if creature.position == position {
                creatures.push(creature);
            }
        }
        creatures
    }

    pub fn num_alive(&self) -> usize {
        let mut num = 0;
        for creature in &self.creatures {
            if creature.is_alive() {
                num += 1;
            }
        }
        num
    }
}

pub struct World {
    pub name: String,
    pub history: Vec<WorldState>,
    pub current_state: WorldState,
    carrying_capacity: usize,
    width: usize,
    height: usize,
    creature_count: usize,
    save_history: bool,
}

impl World {
    pub fn new(
        width: usize,
        height: usize,
        name: String,
        carrying_capacity: usize,
        save_history: bool,
    ) -> Self {
        Self {
            name,
            history: Vec::new(),
            current_state: WorldState::new(),
            carrying_capacity,
            width,
            height,
            creature_count: 0,
            save_history,
        }
    }

    pub fn add_creature_with_position(&mut self, name: &str, position: Position) {
        let creature = Creature::new(
            position,
            Position::new(self.width as i32, self.height as i32),
            name,
            1,
            self.creature_count as u64,
        );
        self.creature_count += 1;
        self.current_state.creatures.push(creature);
    }

    pub fn add_creature(&mut self, name: &str) {
        self.add_creature_with_position(
            name,
            Position::new(self.width as i32 / 2, self.height as i32 / 2),
        );
    }

    pub fn add_creatures_from_world(&mut self, world: World) {
        loop {
            for creature in &world.current_state.creatures {
                if creature.is_alive() {
                    let new_creature = Creature::new_from_old(
                        creature,
                        self.creature_count as u64,
                        Position::new(self.width as i32 / 2, self.height as i32 / 2),
                        Position::new(self.width as i32, self.height as i32),
                    );
                    self.creature_count += 1;
                    self.current_state.creatures.push(new_creature);
                    if self.creature_count >= self.carrying_capacity {
                        return;
                    }
                }
            }
            if self.creature_count == 0 {
                return;
            }
        }
    }

    pub fn simulate(&mut self, n: i32) {
        for _ in 0..n {
            self.step();
        }
    }

    fn step(&mut self) {
        if self.save_history {
            self.history.push(self.current_state.clone());
        }
        for creature in self.current_state.creatures.iter_mut() {
            creature.step(plant_is_here(creature.position));
        }



    }

    pub fn display_map(&self, mode: DisplayMode, states: &[WorldState], mut frame_delay: u64) {
        if mode == DisplayMode::TerminalStatic {
            let state = &states[states.len() - 1];
            let mut map = Map::new(self.width, self.height, self.name.to_string());
            for x in 0..self.width {
                for y in 0..self.height {
                    let position = Position::new(x as i32, y as i32);
                    if plant_is_here(position) {
                        map.set_plant(position);
                    }
                }
            }
            for creature in &state.creatures {
                map.set_creature(creature.position, creature.direction, creature.life as i32);
            }
            map.display();
        } else if mode == DisplayMode::TerminalDynamic {
            enable_raw_mode().unwrap();
            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen).unwrap();
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend).unwrap();

            let mut frame_count = 0;
            let mut is_paused = false;
            let mut show_dead = false;
            let mut cursor = Cursor {
                show: true,
                x: self.width as i32 / 2,
                y: self.height as i32 / 2,
            };
            loop {
                let mut map = Map::new(self.width, self.height, self.name.to_string());
                for creature in &states[frame_count].creatures {
                    map.set_creature(creature.position, creature.direction, creature.life as i32);
                }
                for x in 0..self.width {
                    for y in 0..self.height {
                        let position = Position::new(x as i32, y as i32);
                        if plant_is_here(position) {
                            map.set_plant(position);
                        }
                    }
                }
                cursor.show = is_paused;
                match terminal_graphics::display(
                    &mut terminal,
                    &map,
                    frame_count,
                    frame_delay,
                    &cursor,
                    &states[frame_count],
                    show_dead,
                ) {
                    Interaction::Halt => break,
                    Interaction::Progress => {
                        if !is_paused {
                            frame_count += 1;
                        }
                    }
                    Interaction::Pause => {
                        is_paused = !is_paused;
                    }
                    Interaction::ToggleShowDead => {
                        show_dead = !show_dead;
                    }
                    Interaction::Back => {
                        if frame_count > 0 {
                            frame_count -= 1;
                        }
                    }
                    Interaction::Forward => {
                        frame_count += 1;
                    }
                    Interaction::Up => {
                        cursor.y += 1;
                    }
                    Interaction::Down => {
                        cursor.y -= 1;
                    }
                    Interaction::Left => {
                        cursor.x -= 1;
                    }
                    Interaction::Right => {
                        cursor.x += 1;
                    }
                    Interaction::SpeedUp => {
                        frame_delay -= 10;
                        frame_delay = std::cmp::max(frame_delay, 10);
                    }
                    Interaction::SlowDown => {
                        frame_delay += 10;
                        frame_delay = std::cmp::min(frame_delay, 1000);
                    }
                }
                if frame_count == states.len() {
                    frame_count = 0;
                }
            }

            disable_raw_mode().unwrap();
            execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
            terminal.show_cursor().unwrap();
        }
    }
}
