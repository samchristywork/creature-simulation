pub mod creature;
pub mod genome;
pub mod map;
pub mod plant;
pub mod position;
pub mod terminal_graphics;
pub mod world;

// Log types are error, warn, info, debug, and trace.

use log::{info, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
};
use rand::seq::SliceRandom;

#[derive(PartialEq)]
pub enum DisplayMode {
    TerminalStatic,
    TerminalDynamic,
}

fn main() {
    let logfile = FileAppender::builder()
        .build("/tmp/creature_simulation.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    info!("Simulation has started.");

    let data = String::from_utf8_lossy(include_bytes!("names.in"));
    let names: Vec<&str> = data.split('\n').collect();

    let mut world = world::World::new(80, 30, "World".to_string(), 100);
    for _ in 0..30 {
        world.add_creature(names.choose(&mut rand::thread_rng()).unwrap());
    }
    world.add_plants(100);

    world.simulate(1000);
    world.display_map(DisplayMode::TerminalDynamic, &world.history[0..], 100);
}
