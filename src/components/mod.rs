use specs::prelude::*;

pub mod position;
pub mod rotation;
pub mod velocity;

pub use position::Position;
pub use rotation::Rotation;
pub use velocity::Velocity;

pub fn register_all(world: &mut World) {
    world.register::<Position>();
    world.register::<Rotation>();
    world.register::<Velocity>();
}
