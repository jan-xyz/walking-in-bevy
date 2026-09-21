use bevy::prelude::*;
use lightyear::avian3d::plugin::{AvianReplicationMode, LightyearAvianPlugin};
use lightyear::input::config::InputConfig;
use lightyear::prelude::input::leafwing;
use lightyear::prelude::*;

use crate::player::{CurrentPlayerModel, FacingAngle, Player, PlayerActions, PlayerColor};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    #[allow(deprecated)]
    fn build(&self, app: &mut App) {
        // inputs
        app.add_plugins(leafwing::InputPlugin::<PlayerActions> {
            config: InputConfig {
                rebroadcast_inputs: true,
                ..default()
            },
        });

        // physics
        app.add_plugins(LightyearAvianPlugin {
            replication_mode: AvianReplicationMode::Position {
                sync_to_transform: false,
            },
            rollback_resources: false,
            ..default()
        });

        // components
        app.component::<Player>().replicate().predict();
        app.component::<FacingAngle>().replicate().predict();
        app.component::<PlayerColor>().replicate();
        app.component::<CurrentPlayerModel>().replicate();
    }
}
