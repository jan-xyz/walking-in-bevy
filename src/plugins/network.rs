use avian3d::prelude::*;
use bevy::prelude::*;
use lightyear::avian3d::plugin::{AvianReplicationMode, LightyearAvianPlugin};
use lightyear::input::config::InputConfig;
use lightyear::prelude::input::leafwing;
use lightyear::prelude::*;

use crate::plugins::input::PlayerActions;
use crate::plugins::player::model::{CurrentPlayerModel, PlayerColor};
use crate::plugins::player::Player;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
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
            replication_mode: AvianReplicationMode::Position,
            ..default()
        });

        // components
        app.register_component::<Position>()
            .add_prediction()
            .add_should_rollback(position_should_rollback)
            .add_linear_interpolation()
            .add_linear_correction_fn();

        app.register_component::<Rotation>()
            .add_prediction()
            .add_should_rollback(rotation_should_rollback)
            .add_linear_interpolation()
            .add_linear_correction_fn();

        app.register_component::<LinearVelocity>().add_prediction();

        app.register_component::<AngularVelocity>().add_prediction();

        app.register_component::<CurrentPlayerModel>();
        app.register_component::<PlayerColor>();
        app.register_component::<Player>();
    }
}

fn position_should_rollback(this: &Position, that: &Position) -> bool {
    (this.0 - that.0).length() >= 0.01
}

fn rotation_should_rollback(this: &Rotation, that: &Rotation) -> bool {
    this.angle_between(*that) >= 0.01
}
