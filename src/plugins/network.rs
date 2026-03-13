use avian3d::prelude::*;
use bevy::prelude::*;
use lightyear::avian3d::plugin::{AvianReplicationMode, LightyearAvianPlugin};
use lightyear::frame_interpolation::FrameInterpolationPlugin;
use lightyear::input::config::InputConfig;
use lightyear::prelude::input::leafwing;
use lightyear::prelude::*;

use crate::plugins::input::PlayerActions;
use crate::plugins::player::model::{CurrentPlayerModel, PlayerColor};
use crate::plugins::player::{FacingAngle, Player};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        // inputs
        app.add_plugins(leafwing::InputPlugin::<PlayerActions> {
            config: InputConfig {
                rebroadcast_inputs: false,
                ..default()
            },
        });

        // physics
        app.add_plugins(LightyearAvianPlugin {
            replication_mode: AvianReplicationMode::Position,
            ..default()
        });

        // interpolation
        app.add_plugins(FrameInterpolationPlugin::<Position>::default());
        app.add_plugins(FrameInterpolationPlugin::<FacingAngle>::default());

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

        app.register_component::<LinearVelocity>()
            .add_prediction()
            .add_should_rollback(|_, _| false);

        app.register_component::<AngularVelocity>().add_prediction();

        app.register_component::<CurrentPlayerModel>();
        app.register_component::<PlayerColor>();
        app.register_component::<Player>();
        app.register_component::<FacingAngle>()
            .add_prediction()
            .add_interpolation_with(facing_lerp)
            .add_should_rollback(|_, _| false);
    }
}

fn facing_lerp(start: FacingAngle, other: FacingAngle, t: f32) -> FacingAngle {
    FacingAngle(start.0 + (other.0 - start.0) * t)
}

fn position_should_rollback(this: &Position, that: &Position) -> bool {
    (this.0 - that.0).length() >= 1.5
}

fn rotation_should_rollback(this: &Rotation, that: &Rotation) -> bool {
    this.angle_between(*that) >= 1.5
}
