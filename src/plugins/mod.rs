pub mod camera;
pub mod core;
pub mod input;
pub mod network;
pub mod physics;
pub mod player;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct LocalPlugins;

impl PluginGroup for LocalPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(core::CorePlugin)
            .add(player::PlayerPlugin)
            .add(physics::PhysicsPlugin)
            .add(camera::CameraPlugin)
            .add(input::InputPlugin)
    }
}

pub struct ServerPlugin;

impl PluginGroup for ServerPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(core::CorePlugin)
            .add(player::NetworkPlugin)
            .add(physics::NetworkPlugin)
            .add(network::NetworkPlugin)
    }
}

pub struct ClientPlugin;

impl PluginGroup for ClientPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(core::CorePlugin)
            .add(player::NetworkPlugin)
            .add(physics::NetworkPlugin)
            .add(network::NetworkPlugin)
            .add(camera::CameraPlugin)
    }
}
