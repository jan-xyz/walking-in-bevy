mod model;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::{
    builtins::{TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig},
    prelude::*,
};
use leafwing_input_manager::prelude::ActionState;

use crate::plugins::input::{default_player1_input_map, default_player2_input_map, PlayerAction};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(
            FixedUpdate,
            (
                apply_controls_1.in_set(TnuaUserControlsSystems),
                apply_controls_2.in_set(TnuaUserControlsSystems),
            ),
        );
        app.add_systems(Update, swap_player_model);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Player1;

#[derive(Component)]
pub struct Player2;

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
pub enum PlayerControlScheme {
    Jump(TnuaBuiltinJump),
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut control_scheme_configs: ResMut<Assets<PlayerControlSchemeConfig>>,
) {
    // Player 1
    commands
        .spawn((
            TransformInterpolation,
            Transform::from_xyz(0., 2., 0.),
            // The player character needs to be configured as a dynamic rigid body of the physics
            // engine.
            RigidBody::Dynamic,
            // This is Tnua's interface component.
            TnuaController::<PlayerControlScheme>::default(),
            // The configuration asset can be loaded from a file, but here we are creating it by code
            // and injecting it to the assets resource.
            TnuaConfig::<PlayerControlScheme>(control_scheme_configs.add(
                PlayerControlSchemeConfig {
                    basis: TnuaBuiltinWalkConfig {
                        // The `float_height` must be greater (even if by little) from the distance between
                        // the character's center and the lowest point of its collider.
                        float_height: 1.5,
                        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they
                        // have sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn
                        // what they do.
                        ..Default::default()
                    },
                    jump: TnuaBuiltinJumpConfig {
                        // The height is the only mandatory field of the jump action.
                        height: 4.0,
                        // `TnuaBuiltinJump` also has customization fields with sensible defaults.
                        ..Default::default()
                    },
                },
            )),
            // Tnua can fix the rotation, but the character will still get rotated before it can do so.
            // By locking the rotation we can prevent this.
            LockedAxes::ROTATION_LOCKED,
            // Adding mass here so there are no problems when swapping models.
            Mass(1.0),
            model::CurrentPlayerModel(model::PlayerModelType::Donut),
            Player,
            Player1,
            Name::new("Player1"),
            default_player1_input_map(),
        ))
        .with_children(|parent| {
            model::spawn_player_model(
                parent,
                model::PlayerModelType::Donut,
                &asset_server,
                &mut meshes,
                &mut materials,
            );
        });

    // Player 2
    commands
        .spawn((
            TransformInterpolation,
            Transform::from_xyz(10., 2., 0.),
            // The player character needs to be configured as a dynamic rigid body of the physics
            // engine.
            RigidBody::Dynamic,
            // This is Tnua's interface component.
            TnuaController::<PlayerControlScheme>::default(),
            // The configuration asset can be loaded from a file, but here we are creating it by code
            // and injecting it to the assets resource.
            TnuaConfig::<PlayerControlScheme>(control_scheme_configs.add(
                PlayerControlSchemeConfig {
                    basis: TnuaBuiltinWalkConfig {
                        // The `float_height` must be greater (even if by little) from the distance between
                        // the character's center and the lowest point of its collider.
                        float_height: 1.5,
                        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they
                        // have sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn
                        // what they do.
                        ..Default::default()
                    },
                    jump: TnuaBuiltinJumpConfig {
                        // The height is the only mandatory field of the jump action.
                        height: 4.0,
                        // `TnuaBuiltinJump` also has customization fields with sensible defaults.
                        ..Default::default()
                    },
                },
            )),
            // Tnua can fix the rotation, but the character will still get rotated before it can do so.
            // By locking the rotation we can prevent this.
            LockedAxes::ROTATION_LOCKED,
            // Adding mass here so there are no problems when swapping models.
            Mass(1.0),
            model::CurrentPlayerModel(model::PlayerModelType::Donut),
            Player,
            Player2,
            Name::new("Player2"),
            default_player2_input_map(),
        ))
        .with_children(|parent| {
            model::spawn_player_model(
                parent,
                model::PlayerModelType::Donut,
                &asset_server,
                &mut meshes,
                &mut materials,
            );
        });
}
// Movement System
fn apply_controls_1(
    time: Res<Time>,
    mut query: Query<
        (
            &ActionState<PlayerAction>,
            &mut TnuaController<PlayerControlScheme>,
            &mut Transform,
        ),
        With<Player1>,
    >,
) {
    let Ok((action_state, mut controller, mut transform)) = query.single_mut() else {
        return;
    };
    controller.initiate_action_feeding();

    let forward = transform.forward();
    let mut direction = Vec3::ZERO;
    let rotation_speed = 2.0;

    if action_state.pressed(&PlayerAction::Forward) {
        direction += *forward;
    }
    if action_state.pressed(&PlayerAction::Backward) {
        direction -= *forward;
    }
    if action_state.pressed(&PlayerAction::TurnLeft) {
        transform.rotate_y(rotation_speed * time.delta_secs());
    }
    if action_state.pressed(&PlayerAction::TurnRight) {
        transform.rotate_y(-rotation_speed * time.delta_secs());
    }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis = TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_motion: direction.normalize_or_zero() * 10.0,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    };

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if action_state.pressed(&PlayerAction::Jump) {
        controller.action(PlayerControlScheme::Jump(Default::default()));
    }
}

// Movement System
fn apply_controls_2(
    time: Res<Time>,
    mut query: Query<
        (
            &ActionState<PlayerAction>,
            &mut TnuaController<PlayerControlScheme>,
            &mut Transform,
        ),
        With<Player2>,
    >,
) {
    let Ok((action_state, mut controller, mut transform)) = query.single_mut() else {
        return;
    };
    controller.initiate_action_feeding();

    let forward = transform.forward();
    let mut direction = Vec3::ZERO;
    let rotation_speed = 2.0;

    if action_state.pressed(&PlayerAction::Forward) {
        direction += *forward;
    }
    if action_state.pressed(&PlayerAction::Backward) {
        direction -= *forward;
    }
    if action_state.pressed(&PlayerAction::TurnLeft) {
        transform.rotate_y(rotation_speed * time.delta_secs());
    }
    if action_state.pressed(&PlayerAction::TurnRight) {
        transform.rotate_y(-rotation_speed * time.delta_secs());
    }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis = TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_motion: direction.normalize_or_zero() * 10.0,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    };

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if action_state.pressed(&PlayerAction::Jump) {
        controller.action(PlayerControlScheme::Jump(Default::default()));
    }
}

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
        ),
        With<Player>,
    >,
    model_query: Query<Entity, With<model::PlayerModel>>,
) {
    let Ok((player_entity, action_state, current_player_model)) = player_query.single() else {
        return;
    };

    if action_state.just_pressed(&PlayerAction::SwapModel) {
        for model_entity in model_query.iter() {
            commands.entity(model_entity).despawn();
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
            );
        });

        commands
            .entity(player_entity)
            .insert(model::CurrentPlayerModel(new_model));
    }
}
