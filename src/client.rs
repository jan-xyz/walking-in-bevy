use std::{net::Ipv4Addr, net::SocketAddr, time::Duration};

use avian3d::prelude::{Collider, LockedAxes, Mass, Position, RigidBody};
use bevy::prelude::*;

use bevy_tnua::builtins::{TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig};
use bevy_tnua::prelude::TnuaBuiltinWalk;
use bevy_tnua::{TnuaConfig, TnuaController, TnuaUserControlsSystems};
use leafwing_input_manager::prelude::ActionState;
use lightyear::frame_interpolation::FrameInterpolate;
use lightyear::prelude::*;
use lightyear::{
    netcode::NetcodeClient,
    prelude::{client::*, PeerAddr},
};
use walking_in_bevy::plugins::input::{default_player1_input_map, PlayerActions};
use walking_in_bevy::plugins::player::{
    apply_visual_rotation, debug_forward_gizmo, movement_direction, movement_rotation, FacingAngle,
    Player, PlayerControlScheme, PlayerControlSchemeConfig,
};
use walking_in_bevy::plugins::ClientPlugin;

fn main() {
    let tick_duration = Duration::from_secs_f64(1.0 / 60.0);

    App::new()
        .add_plugins((
            // DefaultPlugins has to be added to correctly derive colliders. This is something that
            // can be changed in the future, by separating the mesh from the collider, e.g.
            // by simplifying to a pill collider. Once that is done we can use MinimalPlugins
            DefaultPlugins.build(),
            ClientPlugins { tick_duration },
            // Game plugins
            ClientPlugin,
            DebugUIPlugin,
        ))
        .add_systems(Startup, connect_to_server)
        .add_systems(FixedUpdate, apply_controls.in_set(TnuaUserControlsSystems))
        .add_systems(PostUpdate, apply_visual_rotation)
        .add_systems(Update, debug_forward_gizmo)
        .add_observer(on_player_added)
        .run();
}

fn on_player_added(
    trigger: On<Add, Player>,
    predicted: Query<(), With<Predicted>>,
    authority: Query<(), With<Controlled>>,
    mut commands: Commands,
    mut control_scheme_configs: ResMut<Assets<PlayerControlSchemeConfig>>,
) {
    if predicted.get(trigger.entity).is_err() {
        return;
    }
    let mut entity_commands = commands.entity(trigger.entity);
    entity_commands.insert((
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
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.0),
        Mass(1.0),
        LockedAxes::ROTATION_LOCKED,
        FrameInterpolate::<Position> {
            trigger_change_detection: true,
            ..default()
        },
        FrameInterpolate::<FacingAngle> {
            trigger_change_detection: true,
            ..default()
        },
        Visibility::default(),
    ));

    if authority.get(trigger.entity).is_ok() {
        entity_commands.insert(default_player1_input_map());
    }
}

fn connect_to_server(mut commands: Commands) {
    let auth = Authentication::Manual {
        server_addr: SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5000),
        client_id: rand::random::<u64>(),
        private_key: [0u8; 32],
        protocol_id: 0,
    };
    let entity = commands
        .spawn((
            NetcodeClient::new(auth, NetcodeConfig::default()).unwrap(),
            PeerAddr(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5000)),
            LocalAddr(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0)),
            UdpIo::default(),
            ReplicationReceiver::default(),
            PredictionManager::default(),
            InputTimelineConfig::new(
                SyncConfig::default(),
                // Make sure we have 1 frame of input delay to allow for prediction and rollbacks.
                InputDelayConfig {
                    minimum_input_delay_ticks: 1,
                    maximum_input_delay_before_prediction: 2,
                    maximum_predicted_ticks: 100,
                },
            ),
        ))
        .id();

    commands.trigger(Connect { entity });
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
        (With<Player>, With<Predicted>),
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
