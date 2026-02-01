use bevy::prelude::*;

use crate::plugins::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(PostUpdate, follow_player);
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

fn follow_player(
    time: Res<Time>,
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };
    let Ok(mut cam_transform) = camera.single_mut() else {
        return;
    };

    let offset = Vec3::new(0., 5., 10.);
    let target = player_transform.translation + offset;
    let t = time.delta_secs() * 30.;
    cam_transform.translation = cam_transform.translation.lerp(target, t);
}
