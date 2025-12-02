use bevy::prelude::*;

mod plugins;
use plugins::GamePlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // framerate pacing
        .add_plugins(bevy_framepace::FramepacePlugin)
        // performance diagnostics
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        // Game plugins
        .add_plugins(GamePlugins)
        .run();
}
