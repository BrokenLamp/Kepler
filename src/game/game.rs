use crate::prelude::*;

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

    pub fn create_entity(&mut self) -> EntityBuilder {
        self.world.create_entity()
    }

    pub fn launch(self) -> ! {
        self.window.start_event_loop(|| {});
    }
}
