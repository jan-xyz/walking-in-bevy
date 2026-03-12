use bevy::prelude::*;

use walking_in_bevy::plugins::LocalPlugins;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Game plugins
            LocalPlugins,
        ))
        .run();
}
