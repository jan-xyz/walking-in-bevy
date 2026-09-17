use avian3d::prelude::*;
use bevy::prelude::*;
use lightyear::avian3d::plugin::{AvianReplicationMode, LightyearAvianPlugin};
use lightyear::input::config::InputConfig;
use lightyear::prelude::input::leafwing;
use lightyear::prelude::*;

use crate::plugins::input::PlayerActions;
use crate::plugins::player::model::{CurrentPlayerModel, PlayerColor};
use crate::plugins::player::{FacingAngle, Player};

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

fn facing_lerp(start: FacingAngle, other: FacingAngle, t: f32) -> FacingAngle {
    FacingAngle(start.0 + (other.0 - start.0) * t)
}

impl Diffable<f32> for FacingAngle {
    fn base_value() -> Self {
        FacingAngle(0.0)
    }

    fn diff(&self, new: &Self) -> f32 {
        new.0 - self.0
    }

    fn apply_diff(&mut self, delta: &f32) {
        self.0 += delta
    }
}
