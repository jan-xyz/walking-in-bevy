use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    // Spawn a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 16., 40.).looking_at(Vec3::new(0., 10., 0.), Vec3::Y),
        Name::new("Main Camera"),
    ));
}
