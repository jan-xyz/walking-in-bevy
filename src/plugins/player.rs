pub mod model;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::{
    builtins::{TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig},
    prelude::*,
};
use leafwing_input_manager::prelude::{ActionState, InputMap};
use serde::{Deserialize, Serialize};

use crate::plugins::input::{default_player1_input_map, default_player2_input_map, PlayerActions};

const ROTATION_SPEED: f32 = 2.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players);
        app.add_systems(FixedUpdate, apply_controls.in_set(TnuaUserControlsSystems));
        app.add_systems(Update, apply_visual_rotation);
        app.add_plugins(model::ModelPlugin);
    }
}

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(model::ModelPlugin);
    }
}

#[derive(Component, PartialEq, Serialize, Deserialize)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Deref, DerefMut)]
pub struct FacingAngle(pub f32);

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
pub enum PlayerControlScheme {
    Jump(TnuaBuiltinJump),
}

struct PlayerConfig {
    name: &'static str,
    spawn_pos: Transform,
    input_map: InputMap<PlayerActions>,
    color: Color,
}

fn spawn_players(
    mut commands: Commands,
    mut control_scheme_configs: ResMut<Assets<PlayerControlSchemeConfig>>,
) {
    let players = [
        PlayerConfig {
            name: "Player 1",
            spawn_pos: Transform::from_xyz(0., 2., 0.),
            input_map: default_player1_input_map(),
            color: Color::Hsla(Hsla::new(180.0, 1.0, 0.5, 1.0)),
        },
        PlayerConfig {
            name: "Player 2",
            spawn_pos: Transform::from_xyz(10., 2., 0.),
            input_map: default_player2_input_map(),
            color: Color::Hsla(Hsla::new(100.0, 1.0, 0.5, 1.0)),
        },
    ];

    for PlayerConfig {
        name,
        spawn_pos,
        input_map,
        color,
    } in players
    {
        commands
            .spawn(player_bundle(
                name,
                spawn_pos,
                color,
                &mut control_scheme_configs,
            ))
            .insert(input_map);
    }
}

pub fn player_bundle(
    name: &'static str,
    spawn_pos: Transform,
    color: Color,
    control_scheme_configs: &mut Assets<PlayerControlSchemeConfig>,
) -> impl Bundle {
    (
        spawn_pos,
        Name::new(name),
        model::PlayerColor(color),
        TransformInterpolation,
        RigidBody::Dynamic,
        TnuaController::<PlayerControlScheme>::default(),
        TnuaConfig::<PlayerControlScheme>(control_scheme_configs.add(PlayerControlSchemeConfig {
            basis: TnuaBuiltinWalkConfig {
                // The `float_height` must be greater (even if by little) from the distance between
                // the character's center and the lowest point of its collider.
                float_height: 1.5,
                ..Default::default()
            },
            jump: TnuaBuiltinJumpConfig {
                height: 4.0,
                ..Default::default()
            },
        })),
        // Tnua can fix the rotation, but the character will still get rotated before it can do so.
        // By locking the rotation we can prevent this.
        LockedAxes::ROTATION_LOCKED,
        // Adding mass & collider so there are no problems when swapping models.
        Mass(1.0),
        Collider::capsule(0.5, 1.0),
        model::CurrentPlayerModel(model::PlayerModelType::Donut),
        Player,
        FacingAngle(0.0),
        Visibility::default(),
    )
}

#[allow(clippy::type_complexity)]
pub fn apply_controls(
    time: Res<Time<Fixed>>,
    mut query: Query<
        (
            &ActionState<PlayerActions>,
            &mut TnuaController<PlayerControlScheme>,
            &mut FacingAngle,
        ),
        With<Player>,
    >,
) {
    for (action_state, mut controller, mut facing) in query.iter_mut() {
        controller.initiate_action_feeding();

        // Rotation
        let left_pressed = action_state.pressed(&PlayerActions::TurnLeft);
        let right_pressed = action_state.pressed(&PlayerActions::TurnRight);
        facing.0 += movement_rotation(time.delta_secs(), left_pressed, right_pressed);

        // Direction
        let forward = Quat::from_rotation_y(facing.0) * Vec3::NEG_Z;
        let forward_pressed = action_state.pressed(&PlayerActions::Forward);
        let backward_pressed = action_state.pressed(&PlayerActions::Backward);
        let direction = movement_direction(forward, forward_pressed, backward_pressed);

        controller.basis = TnuaBuiltinWalk {
            desired_motion: direction.normalize_or_zero() * 10.0,
            ..Default::default()
        };

        // Jumping
        if action_state.pressed(&PlayerActions::Jump) {
            controller.action(PlayerControlScheme::Jump(Default::default()));
        }
    }
}

pub fn apply_visual_rotation(
    players: Query<(&FacingAngle, &Children), With<Player>>,
    mut models: Query<&mut Transform, With<model::PlayerModel>>,
) {
    for (facing, children) in players.iter() {
        for child in children.iter() {
            if let Ok(mut model_transform) = models.get_mut(child) {
                model_transform.rotation = Quat::from_rotation_y(facing.0);
            }
        }
    }
}

pub fn debug_forward_gizmo(
    mut gizmos: Gizmos,
    players: Query<(&Transform, &FacingAngle), With<Player>>,
) {
    for (transform, facing) in players.iter() {
        let forward = Quat::from_rotation_y(facing.0) * Vec3::NEG_Z;
        let start = transform.translation + Vec3::Y * 1.5;
        gizmos.line(start, start + forward * 5.0, Color::srgb(1.0, 0.0, 1.0));
    }
}

pub fn movement_direction(forward: Vec3, forward_pressed: bool, backward_pressed: bool) -> Vec3 {
    let mut direction = Vec3::ZERO;
    if forward_pressed {
        direction += forward;
    }
    if backward_pressed {
        direction -= forward;
    }
    direction
}

pub fn movement_rotation(time_delta_sec: f32, left_pressed: bool, right_pressed: bool) -> f32 {
    let mut rotation: f32 = 0.0;
    if left_pressed {
        rotation += ROTATION_SPEED * time_delta_sec;
    }
    if right_pressed {
        rotation -= ROTATION_SPEED * time_delta_sec;
    }
    rotation
}

#[cfg(test)]
mod test {
    use super::*;

    use bevy::math::Vec3;

    #[test]
    fn test_movement_direction() {
        struct TestCase {
            name: &'static str,
            forward_pressed: bool,
            backward_pressed: bool,
            expected: Vec3,
        }

        let test_cases = [
            TestCase {
                name: "no keys pressed -> Zero vec",
                forward_pressed: false,
                backward_pressed: false,
                expected: Vec3::ZERO,
            },
            TestCase {
                name: "forward only pressed -> equal as forward",
                forward_pressed: true,
                backward_pressed: false,
                expected: Vec3::Z,
            },
            TestCase {
                name: "backward only pressed -> neg of forward",
                forward_pressed: false,
                backward_pressed: true,
                expected: Vec3::NEG_Z,
            },
            TestCase {
                name: "both pressed -> cancel out to zero",
                forward_pressed: true,
                backward_pressed: true,
                expected: Vec3::ZERO,
            },
        ];

        for TestCase {
            name,
            forward_pressed,
            backward_pressed,
            expected,
        } in test_cases
        {
            let actual = movement_direction(Vec3::Z, forward_pressed, backward_pressed);

            assert_eq!(
                actual, expected,
                "Test {name} failed: expected: {expected}, actual: {actual}"
            )
        }
    }

    #[test]
    fn test_movement_rotation() {
        struct TestCase {
            name: &'static str,
            left_pressed: bool,
            right_pressed: bool,
            expected: f32,
        }

        let test_cases = [
            TestCase {
                name: "no keys pressed",
                left_pressed: false,
                right_pressed: false,
                expected: 0.0,
            },
            TestCase {
                name: "left only pressed -> positive rotation",
                left_pressed: true,
                right_pressed: false,
                expected: 2.0,
            },
            TestCase {
                name: "right only pressed -> negative rotation",
                left_pressed: false,
                right_pressed: true,
                expected: -2.0,
            },
            TestCase {
                name: "both pressed -> cancel out",
                left_pressed: true,
                right_pressed: true,
                expected: 0.0,
            },
        ];

        for TestCase {
            name,
            left_pressed,
            right_pressed,
            expected,
        } in test_cases
        {
            let actual = movement_rotation(1.0, left_pressed, right_pressed);

            assert_eq!(
                actual, expected,
                "Test {name} failed: expected: {expected}, actual: {actual}"
            )
        }
    }
}
