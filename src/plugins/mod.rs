pub mod camera;
pub mod core;
pub mod physics;
pub mod player;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(core::CorePlugin)
            .add(player::PlayerPlugin)
            .add(physics::PhysicsPlugin)
            .add(camera::CameraPlugin)
    }
}
