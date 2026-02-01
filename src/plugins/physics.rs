use crate::plugins::player::PlayerControlScheme;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::TnuaControllerPlugin;
use bevy_tnua_avian3d::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // Physics library
            PhysicsPlugins::default(),
            // Character controller
            TnuaControllerPlugin::<PlayerControlScheme>::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
        ));
    }
}
