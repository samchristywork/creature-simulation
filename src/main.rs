pub mod creature;
pub mod genome;
pub mod map;
pub mod plant;
pub mod position;
pub mod terminal_graphics;
pub mod world;

use rand::seq::SliceRandom;
use std::time::Duration;

#[derive(PartialEq)]
pub enum DisplayMode {
    TerminalStatic,
    TerminalDynamic,
}

fn main() {
    let data = String::from_utf8_lossy(include_bytes!("names.in"));
    let names: Vec<&str> = data.split('\n').collect();

    let mut world = world::World::new(80, 30, "World".to_string());
    for _ in 0..100 {
        world.add_creature(names.choose(&mut rand::thread_rng()).unwrap());
    }
    world.add_plants(100);

    world.simulate(1000);
    world.display_map(
        DisplayMode::TerminalDynamic,
        &world.history[0..],
        Duration::from_millis(100),
    );
}
