use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

mod plugins;
use plugins::GamePlugins;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // framerate pacing
            bevy_framepace::FramepacePlugin,
            // performance diagnostics
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            bevy::diagnostic::EntityCountDiagnosticsPlugin,
            bevy::diagnostic::SystemInformationDiagnosticsPlugin,
            PerfUiPlugin,
            // Game plugins
            GamePlugins,
        ))
        .run();
}
