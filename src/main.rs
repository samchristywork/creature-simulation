pub mod creature;
pub mod genome;
pub mod map;
pub mod plant;
pub mod position;
pub mod terminal_graphics;
pub mod world;

use std::time::Duration;

#[derive(PartialEq)]
pub enum DisplayMode {
    TerminalStatic,
    TerminalDynamic,
}

fn main() {
    let mut world = world::World::new(80, 30, "World".to_string());
    world.add_creatures(100);
    world.add_plants(100);

    world.simulate(1000);
    world.display_map(
        DisplayMode::TerminalDynamic,
        &world.history[0..],
        Duration::from_millis(100),
    );
}
