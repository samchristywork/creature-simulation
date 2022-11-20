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
    let carrying_capacity = 100;
    let generations = 100;
    let simulation_steps = 1000;

    let mut world1 = world::World::new(80, 30, "World".to_string(), carrying_capacity, false);
    for _ in 0..carrying_capacity {
        world1.add_creature(names.choose(&mut rand::thread_rng()).unwrap());
    }
    for generation in 0..generations {
        let mut world2 = world::World::new(80, 30, "World".to_string(), carrying_capacity, false);
        world2.add_creatures_from_world(world1);
        world2.simulate(simulation_steps);
        world1 = world2;

        info!(
            "{} creatures survived generation {} ({}%).",
            world1.current_state.num_alive(),
            generation,
            100 * world1.current_state.num_alive() / carrying_capacity,
        );
    }

    let mut world2 = world::World::new(80, 30, "World".to_string(), carrying_capacity, true);
    world2.add_creatures_from_world(world1);
    world2.simulate(simulation_steps);
    world1 = world2;

    info!("Simulation has Ended.");

    world1.display_map(DisplayMode::TerminalDynamic, &world1.history[0..], 10);
}
