use std::{net::Ipv4Addr, net::SocketAddr, time::Duration};

use bevy::{app::ScheduleRunnerPlugin, prelude::*};

use leafwing_input_manager::prelude::ActionState;
use lightyear::prelude::{
    server::*, Connected, ControlledBy, LocalAddr, NetworkTarget, PredictionTarget, Replicate,
    ReplicationSender, SendUpdatesMode,
};
use walking_in_bevy::plugins::{
    input::PlayerActions,
    player::{player_bundle, PlayerControlSchemeConfig},
    ServerPlugin,
};

fn main() {
    let tick_duration = Duration::from_secs_f64(1.0 / 60.0);

    App::new()
        .add_plugins((
            // DefaultPlugins has to be added to correctly derive colliders. This is something that
            // can be changed in the future, by separating the mesh from the collider, e.g.
            // by simplifying to a pill collider. Once that is done we can use MinimalPlugins
            DefaultPlugins.build().disable::<bevy::winit::WinitPlugin>(),
            ScheduleRunnerPlugin::default(),
            ServerPlugins { tick_duration },
            // Game plugins
            ServerPlugin,
        ))
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
) {
    commands
        .spawn(player_bundle(
            "Player 1",
            Transform::from_xyz(10.0, 2.0, 0.0),
            Color::Hsla(Hsla::new(100.0, 1.0, 0.5, 1.0)),
            &mut control_scheme_configs,
        ))
        .insert((
            Replicate::to_clients(NetworkTarget::All),
            PredictionTarget::to_clients(NetworkTarget::All),
            ControlledBy {
                owner: trigger.entity,
                lifetime: default(),
            },
            ActionState::<PlayerActions>::default(),
        ));
    info!("Connected a client!");
}
