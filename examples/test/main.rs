use kepler::prelude::*;

fn main() {
    let mut game = Game::new("Kepler Engine");

    game.create_entity()
        .with(components::Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
        .build();

    game.launch();
}
