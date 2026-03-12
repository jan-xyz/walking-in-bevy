mod model;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::{
    builtins::{TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig},
    prelude::*,
};
use leafwing_input_manager::prelude::{ActionState, InputMap};

use crate::plugins::input::{default_player1_input_map, default_player2_input_map, PlayerAction};

const ROTATION_SPEED: f32 = 2.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players);
        app.add_systems(FixedUpdate, apply_controls.in_set(TnuaUserControlsSystems));
        app.add_systems(Update, swap_player_model);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
pub enum PlayerControlScheme {
    Jump(TnuaBuiltinJump),
}

struct PlayerConfig {
    name: &'static str,
    spawn_pos: Transform,
    input_map: InputMap<PlayerAction>,
    color: Color,
}

fn spawn_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

    for config in players {
        commands
            .spawn((
                config.spawn_pos,
                Name::new(config.name),
                config.input_map,
                model::PlayerColor(config.color),
                TransformInterpolation,
                RigidBody::Dynamic,
                TnuaController::<PlayerControlScheme>::default(),
                TnuaConfig::<PlayerControlScheme>(control_scheme_configs.add(
                    PlayerControlSchemeConfig {
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
                    },
                )),
                // Tnua can fix the rotation, but the character will still get rotated before it can do so.
                // By locking the rotation we can prevent this.
                LockedAxes::ROTATION_LOCKED,
                // Adding mass & collider so there are no problems when swapping models.
                Mass(1.0),
                Collider::capsule(0.5, 1.0),
                model::CurrentPlayerModel(model::PlayerModelType::Donut),
                Player,
                Visibility::default(),
            ))
            .with_children(|parent| {
                model::spawn_player_model(
                    parent,
                    model::PlayerModelType::Donut,
                    &asset_server,
                    &mut meshes,
                    &mut materials,
                    config.color,
                );
            });
    }
}

#[allow(clippy::type_complexity)]
fn apply_controls(
    time: Res<Time>,
    mut query: Query<
        (
            &ActionState<PlayerAction>,
            &mut TnuaController<PlayerControlScheme>,
            &mut Transform,
        ),
        With<Player>,
    >,
) {
    for (action_state, mut controller, mut transform) in query.iter_mut() {
        controller.initiate_action_feeding();

        // Direction
        let forward = transform.forward();
        let forward_pressed = action_state.pressed(&PlayerAction::Forward);
        let backward_pressed = action_state.pressed(&PlayerAction::Backward);
        let direction = movement_direction(forward, forward_pressed, backward_pressed);

        // Rotation
        let left_pressed = action_state.pressed(&PlayerAction::TurnLeft);
        let right_pressed = action_state.pressed(&PlayerAction::TurnRight);
        let rotation = movement_rotation(time.delta_secs(), left_pressed, right_pressed);
        transform.rotate_y(rotation);

        controller.basis = TnuaBuiltinWalk {
            desired_motion: direction.normalize_or_zero() * 10.0,
            ..Default::default()
        };

        // Jumping
        if action_state.pressed(&PlayerAction::Jump) {
            controller.action(PlayerControlScheme::Jump(Default::default()));
        }
    }
}

fn movement_direction(forward: Dir3, forward_pressed: bool, backward_pressed: bool) -> Vec3 {
    let mut direction = Vec3::ZERO;
    if forward_pressed {
        direction += *forward;
    }
    if backward_pressed {
        direction -= *forward;
    }
    direction
}

fn movement_rotation(time_delta_sec: f32, left_pressed: bool, right_pressed: bool) -> f32 {
    let mut rotation: f32 = 0.0;
    if left_pressed {
        rotation += ROTATION_SPEED * time_delta_sec;
    }
    if right_pressed {
        rotation -= ROTATION_SPEED * time_delta_sec;
    }
    rotation
}

#[allow(clippy::type_complexity)]
fn swap_player_model(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<
        (
            Entity,
            &ActionState<PlayerAction>,
            &model::CurrentPlayerModel,
            &Children,
            &model::PlayerColor,
        ),
        With<Player>,
    >,
    model_query: Query<Entity, With<model::PlayerModel>>,
) {
    for (player_entity, action_state, current_player_model, childern, color) in player_query.iter()
    {
        if action_state.just_pressed(&PlayerAction::SwapModel) {
            for model_entity in model_query.iter() {
                if childern.contains(&model_entity) {
                    commands.entity(model_entity).despawn();
                }
            }

            let new_model = match current_player_model.0 {
                model::PlayerModelType::Donut => model::PlayerModelType::Cube,
                model::PlayerModelType::Cube => model::PlayerModelType::Donut,
            };

            commands.entity(player_entity).with_children(|parent| {
                model::spawn_player_model(
                    parent,
                    new_model,
                    &asset_server,
                    &mut meshes,
                    &mut materials,
                    color.0,
                );
            });

            commands
                .entity(player_entity)
                .insert(model::CurrentPlayerModel(new_model));
        }
    }
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
            let actual = movement_direction(Dir3::Z, forward_pressed, backward_pressed);

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
