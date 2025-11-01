use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // Physics library
            PhysicsPlugins::default(),
            // Character controller
            TnuaControllerPlugin::default(),
            TnuaAvian3dPlugin::default(),
        ));
    }
}
