use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::{
    builtins::{TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig},
    prelude::*,
};
use bevy_tnua_avian3d::*;
use leafwing_input_manager::prelude::ActionState;

use crate::plugins::input::{default_player_input_map, PlayerAction};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, apply_controls.in_set(TnuaUserControlsSystems));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
pub enum PlayerControlScheme {
    Jump(TnuaBuiltinJump),
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut control_scheme_configs: ResMut<Assets<PlayerControlSchemeConfig>>,
) {
    // Add a physics body using Avian 3D
    commands.spawn((
        TransformInterpolation,
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("export.glb"))),
        Transform::from_xyz(0., 2., 0.),
        // The player character needs to be configured as a dynamic rigid body of the physics
        // engine.
        RigidBody::Dynamic,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        // This is Tnua's interface component.
        TnuaController::<PlayerControlScheme>::default(),
        // A sensor shape is not strictly necessary, but without it we'll get weird results.
        TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
        // The configuration asset can be loaded from a file, but here we are creating it by code
        // and injecting it to the assets resource.
        TnuaConfig::<PlayerControlScheme>(control_scheme_configs.add(PlayerControlSchemeConfig {
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
        })),
        // Tnua can fix the rotation, but the character will still get rotated before it can do so.
        // By locking the rotation we can prevent this.
        LockedAxes::ROTATION_LOCKED,
        Name::new("Player"),
        Player,
        default_player_input_map(),
    ));
}

// Movement System
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
