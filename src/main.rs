use bevy::prelude::*;

mod plugins;
use plugins::GamePlugins;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Game plugins
            GamePlugins,
        ))
        .run();
}
