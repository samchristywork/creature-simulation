pub mod creature;
pub mod genome;
pub mod map;
pub mod position;
pub mod terminal_graphics;
pub mod world;

// Log types are error, warn, info, debug, and trace.

use log::info;
use rand::seq::SliceRandom;

#[derive(PartialEq, Eq)]
pub enum DisplayMode {
    TerminalStatic,
    TerminalDynamic,
}

fn main() {
    log4rs::init_file("src/log4rs.yaml", Default::default()).unwrap();

    info!("Simulation has started.");

    let data = String::from_utf8_lossy(include_bytes!("names.in"));
    let names: Vec<&str> = data.split('\n').collect();

    let mut world1 = world::World::new(80, 30, "World".to_string(), 100, false);
    for _ in 0..100 {
        world1.add_creature(names.choose(&mut rand::thread_rng()).unwrap());
    }
    world1.simulate(1000);
    info!(
        "{} creatures survived the generation.",
        world1.current_state.num_alive()
    );

    let mut world2 = world::World::new(80, 30, "World".to_string(), 100, true);
    world2.add_creatures_from_world(world1);
    world2.simulate(1000);

    world2.display_map(DisplayMode::TerminalDynamic, &world2.history[0..], 100);
}
