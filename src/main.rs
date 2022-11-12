pub mod creature;
pub mod plant;
pub mod position;
pub mod world;

fn main() {
    let mut world = world::World::new();
    world.add_creatures(100);
    world.add_plants(100);
    world.simulate(100);
    world.display_results();
}
