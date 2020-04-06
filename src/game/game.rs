use crate::prelude::*;
use specs::prelude::*;

pub struct Game {
    world: World,
    window: Window,
}

impl Game {
    pub fn new(name: &str) -> Game {
        let mut world = World::new();
        components::register_all(&mut world);
        let window = Window::new(name);
        Game { world, window }
    }
}
