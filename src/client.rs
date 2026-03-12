use std::{net::Ipv4Addr, net::SocketAddr, time::Duration};

use bevy::prelude::*;

use lightyear::prelude::*;
use lightyear::{
    netcode::NetcodeClient,
    prelude::{client::*, PeerAddr},
};
use walking_in_bevy::plugins::input::default_player1_input_map;
use walking_in_bevy::plugins::player::Player;
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
        ))
        .add_systems(Startup, connect_to_server)
        .add_observer(on_player_added)
        .run();
}

fn on_player_added(trigger: On<Add, Player>, mut commands: Commands) {
    commands
        .entity(trigger.entity)
        .insert(default_player1_input_map());
}

fn connect_to_server(mut commands: Commands) {
    let auth = Authentication::Manual {
        server_addr: SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 5000),
        client_id: 0, // unique per client
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
        ))
        .id();

    commands.trigger(Connect { entity });
}
