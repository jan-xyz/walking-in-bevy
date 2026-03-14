use std::{net::Ipv4Addr, net::SocketAddr, time::Duration};

use bevy::{prelude::*, scene::ScenePlugin};

use bevy_tnua::TnuaUserControlsSystems;
use leafwing_input_manager::prelude::ActionState;
use lightyear::prelude::{
    server::*, Connected, ControlledBy, InterpolationTarget, LocalAddr, NetworkTarget,
    PredictionTarget, RemoteId, Replicate, ReplicationSender, SendUpdatesMode,
};
use walking_in_bevy::plugins::{
    input::PlayerActions,
    player::{apply_controls, player_bundle, Player, PlayerControlSchemeConfig},
    ServerPlugin,
};

fn main() {
    let tick_duration = Duration::from_secs_f64(1.0 / 60.0);

    App::new()
        .add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            TransformPlugin,
            ScenePlugin,
            ServerPlugins { tick_duration },
            // Game plugins
            ServerPlugin,
        ))
        .init_asset::<Mesh>()
        .add_systems(FixedUpdate, apply_controls.in_set(TnuaUserControlsSystems))
        .add_systems(Startup, start_server)
        .add_observer(on_client_connected)
        .add_observer(on_new_client)
        .run();
}

fn start_server(mut commands: Commands) {
    let entity = commands
        .spawn((
            NetcodeServer::new(NetcodeConfig::default()),
            LocalAddr(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 5000)),
            ServerUdpIo::default(),
        ))
        .id();

    commands.trigger(Start { entity });
}

fn on_new_client(trigger: On<Add, LinkOf>, mut commands: Commands) {
    commands
        .entity(trigger.entity)
        .insert(ReplicationSender::new(
            Duration::from_millis(100),
            SendUpdatesMode::SinceLastAck,
            false,
        ));
}

fn on_client_connected(
    trigger: On<Add, Connected>,
    mut commands: Commands,
    mut control_scheme_configs: ResMut<Assets<PlayerControlSchemeConfig>>,
    players: Query<(), With<Player>>,
    client_query: Query<&RemoteId, With<ClientOf>>,
) {
    let Ok(remote_id) = client_query.get(trigger.entity) else {
        return;
    };
    let client_id = remote_id.0;
    let player_index = players.iter().len();

    let configs = [
        (
            "Player 1",
            Vec3::new(0.0, 2.0, 0.0),
            Hsla::new(180.0, 1.0, 0.5, 1.0),
        ),
        (
            "Player 2",
            Vec3::new(10.0, 2.0, 0.0),
            Hsla::new(100.0, 1.0, 0.5, 1.0),
        ),
    ];
    let (name, pos, color) = configs[player_index % configs.len()];
    commands
        .spawn(player_bundle(
            name,
            Transform::from_translation(pos),
            Color::Hsla(color),
            &mut control_scheme_configs,
        ))
        .insert((
            Replicate::to_clients(NetworkTarget::All),
            PredictionTarget::to_clients(NetworkTarget::Single(client_id)),
            InterpolationTarget::to_clients(NetworkTarget::AllExceptSingle(client_id)),
            ControlledBy {
                owner: trigger.entity,
                lifetime: default(),
            },
            ActionState::<PlayerActions>::default(),
        ));
    info!("Connected a client!");
}
